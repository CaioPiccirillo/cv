use image::{GrayImage, Rgb};
use imageproc::{
    edges::canny,
    hough::{detect_lines, draw_polar_lines, LineDetectionOptions, PolarLine},
    map::map_colors,
};
use nalgebra::{Matrix2, Point2, Vector2};

trait Pattern {
    fn find(&self, image: &GrayImage) -> Vec<Point2<f32>>;
}
struct ChessBoard {
    size: (u32, u32),
    square_size: f32,
}

impl ChessBoard {
    pub fn new(size: (u32, u32), square_size: f32) -> ChessBoard {
        ChessBoard {
            size: size,
            square_size: square_size,
        }
    }
}

impl Pattern for ChessBoard {
    /// Finds the points of the chessboard pattern in the image.
    fn find(&self, gray_image: &GrayImage) -> Vec<Point2<f32>> {
        let mut points = Vec::new();
        // Detect edges usings canny
        let edges = canny(gray_image, 127.0, 255.0);
        // Detect lines using Hough transform
        let options = LineDetectionOptions {
            vote_threshold: 100,
            suppression_radius: 10,
        };
        let lines: Vec<PolarLine> = detect_lines(&edges, options);

        let white = Rgb::<u8>([255, 255, 255]);
        let green = Rgb::<u8>([0, 255, 0]);
        let black = Rgb::<u8>([0, 0, 0]);

        // Convert edge image to colour
        let color_edges = map_colors(&edges, |p| if p[0] > 0 { white } else { black });

        // Draw lines on top of edge image
        let lines_image = draw_polar_lines(&color_edges, &lines, green); // FIXME: Remove this drawing

        // Find intersections of lines
        for i in 0..lines.len() {
            for j in i..lines.len() {
                // Due to Hough algorithm, the radius of the lines can be negative and shall be considered on angle
                let alpha = if lines[i].r >= 0. {
                    lines[i].angle_in_degrees as f32
                } else {
                    lines[i].angle_in_degrees as f32 + 180.0
                };
                let beta = if lines[j].r >= 0. {
                    lines[j].angle_in_degrees as f32
                } else {
                    lines[j].angle_in_degrees as f32 + 180.0
                };
                #[rustfmt::skip]
                let a: Matrix2<f32> = Matrix2::new(
                    alpha.to_radians().cos(), alpha.to_radians().sin(),
                    beta.to_radians().cos(), beta.to_radians().sin(),
                );
                let b: Vector2<f32> = Vector2::new(lines[i].r.abs(), lines[j].r.abs());
                let decomp = a.lu();
                let x = decomp.solve(&b);
                // Check if there's a solution
                match x {
                    Some(x) => {
                        let point = Point2::new(x[0], x[1]);
                        // Remove points that are outside the image
                        if point.x <= gray_image.dimensions().0 as f32
                            && point.y <= gray_image.dimensions().1 as f32
                            && point.x >= 0.0
                            && point.y >= 0.0
                        {
                            points.push(point);
                        }
                    }
                    None => continue,
                }
            }
        }

        lines_image.save("../res/lines.png").unwrap();
        points
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::Rgb;
    use imageproc::drawing::draw_cross_mut;
    #[test]
    fn show_chess_board() {
        let src_image = image::open("../res/left01.jpg").expect("Error opening image");
        let chess_board = ChessBoard::new((6, 9), 0.065);
        let gray_image = src_image.to_luma8();
        let corners = chess_board.find(&gray_image);
        let mut image_buffer = src_image.to_rgb8();
        for i in 0..corners.len() {
            draw_cross_mut(
                &mut image_buffer,
                Rgb::<u8>([255, 0, 0]),
                corners[i][0] as i32,
                corners[i][1] as i32,
            );
        }
        image_buffer.save("../res/chess_board.png").unwrap();
        println!("{:?}", corners);
    }
}
