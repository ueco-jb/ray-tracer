use crate::tuple::*;

struct Ray {
    origin: Tuple,
    direction: Tuple,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_and_querrying_ray() {
        let origin = point(1.0, 2.0, 3.0);
        let direction = vector(4.0, 5.0, 6.0);

        let r = Ray { origin, direction };

        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }
}
