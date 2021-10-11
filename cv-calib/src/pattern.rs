use image::{GrayImage, Rgb};
use imageproc::edges::canny;
use imageproc::hough::{detect_lines, draw_polar_lines, LineDetectionOptions, PolarLine};
use imageproc::map::map_colors;
struct ChessBoard {
    size: (u32, u32),
    square_size: f32,
}

impl ChessBoard {
    fn new(size: (u32, u32), square_size: f32) -> ChessBoard {
        ChessBoard {
            size: size,
            square_size: square_size,
        }
    }

    fn find(&self, gray_image: &GrayImage) -> Vec<(u32, u32)> {
        let mut corners = Vec::new();
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
        let lines_image = draw_polar_lines(&color_edges, &lines, green);
        lines_image.save("../res/lines.png").unwrap();
        corners
    }
}

#[cfg(test)]
mod tests {
    use super::ChessBoard;

    #[test]
    fn show_chess_board() {
        let src_image = image::open("../res/left01.jpg").expect("Error opening image");
        let chess_board = ChessBoard::new((6, 9), 0.065);
        let corners = chess_board.find(&src_image.to_luma8());
        println!("{:?}", corners);
    }
}
