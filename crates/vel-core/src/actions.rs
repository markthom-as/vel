pub const OBJECT_GET: &str = "object.get";
pub const OBJECT_QUERY: &str = "object.query";
pub const OBJECT_CREATE: &str = "object.create";
pub const OBJECT_UPDATE: &str = "object.update";
pub const OBJECT_DELETE: &str = "object.delete";
pub const OBJECT_LINK: &str = "object.link";
pub const OBJECT_EXPLAIN: &str = "object.explain";

pub fn generic_object_action_names() -> [&'static str; 7] {
    [
        OBJECT_GET,
        OBJECT_QUERY,
        OBJECT_CREATE,
        OBJECT_UPDATE,
        OBJECT_DELETE,
        OBJECT_LINK,
        OBJECT_EXPLAIN,
    ]
}

#[cfg(test)]
mod tests {
    use super::generic_object_action_names;

    #[test]
    fn generic_action_vocabulary_is_stable() {
        assert_eq!(
            generic_object_action_names(),
            [
                "object.get",
                "object.query",
                "object.create",
                "object.update",
                "object.delete",
                "object.link",
                "object.explain",
            ]
        );
    }
}

