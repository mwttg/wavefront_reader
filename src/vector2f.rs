#[derive(PartialEq, Debug)]
pub struct Vector2f {
    x: f64,
    y: f64
}

pub fn create_from(line: &str) -> Vector2f {
    let parts: Vec<&str> = line.split(crate::SPACE).collect();

    Vector2f {
        x: parts[1].parse().unwrap(),
        y: parts[2].parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::vector2f::{create_from, Vector2f};

    #[test]
    fn create_from_valid() {
        let line = "token 1.234 9.678";
        let actual = create_from(line);
        let expected = Vector2f { x: 1.234, y: 9.678 };

        assert_eq!(actual, expected);
    }

    #[test]
    #[should_panic]
    fn create_from_invalid_value() {
        let line = "token 1.234 invalid";
        create_from(line);
    }

    #[test]
    #[should_panic]
    fn create_from_too_few_values() {
        let line = "token 1.234";
        create_from(line);
    }
}