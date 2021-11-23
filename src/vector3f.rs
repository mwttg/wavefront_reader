static SPACE: &str = " ";

#[derive(PartialEq, Debug)]
pub struct Vector3f {
    x: f64,
    y: f64,
    z: f64,
}

pub fn create_from(line: &str) -> Vector3f {
    let parts: Vec<&str> = line.split(SPACE).collect();

    Vector3f {
        x: parts[1].parse().unwrap(),
        y: parts[2].parse().unwrap(),
        z: parts[3].parse().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use crate::vector3f::{create_from, Vector3f};

    #[test]
    fn create_from_valid() {
        let line = "token 1.234 9.678 4.321";
        let actual = create_from(line);
        let expected = Vector3f { x: 1.234, y: 9.678, z: 4.321 };

        assert_eq!(actual, expected);
    }

    #[test]
    #[should_panic]
    fn create_from_invalid_value() {
        let line = "token 1.234 invalid 4.321";
        create_from(line);
    }

    #[test]
    #[should_panic]
    fn create_from_too_few_values() {
        let line = "token 1.234 5.678";
        create_from(line);
    }
}
