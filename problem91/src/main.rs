struct Vector {
    dx: i32,
    dy: i32,
}

impl Vector {
    fn from(dx: i32, dy: i32) -> Vector {
        Vector { dx, dy }
    }

    fn is_orthogonal(&self, other: &Vector) -> bool {
        self.dx * other.dx == -(self.dy * other.dy)
    }
}

#[test]
fn orthogonal_test() {
    assert!(Vector::from(2, 3).is_orthogonal(&Vector::from(3, -2)));
}

#[test]
fn orthogonal_test_different_scalar() {
    assert!(Vector::from(4, 6).is_orthogonal(&Vector::from(-3, 2)));
}

#[test]
fn non_orthogonal_test() {
    assert!(!Vector::from(1, 1).is_orthogonal(&Vector::from(3, 2)));
}

fn count_right_angled_triangles(side_size: u32) -> u32 {
    let side: i32 = side_size as i32 + 1;
    let mut count = 0;
    for xy1 in 1..(side * side) {
        for xy2 in 1..xy1 {
            let x1 = xy1 % side;
            let y1 = xy1 / side;

            let x2 = xy2 % side;
            let y2 = xy2 / side;

            let a = Vector::from(x1, y1);
            let b = Vector::from(x2, y2);
            let c = Vector::from(x1 - x2, y1 - y2);
            if a.is_orthogonal(&b) || b.is_orthogonal(&c) || c.is_orthogonal(&a) {
                assert!(a.is_orthogonal(&b) ^ b.is_orthogonal(&c) ^ c.is_orthogonal(&a));
                count += 1;
            }
        }
    }
    count
}

#[test]
fn right_triangles_in_coord_less_than_2() {
    assert_eq!(count_right_angled_triangles(2), 14);
}

#[test]
fn right_triangles_in_coord_less_than_3() {
    assert_eq!(count_right_angled_triangles(3), 33);
}

fn main() {
    println!("{}", count_right_angled_triangles(50));
}
