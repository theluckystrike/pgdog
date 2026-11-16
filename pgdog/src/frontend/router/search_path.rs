use crate::{backend::Schema, net::parameter::ParameterValue};

#[derive(Debug)]
pub(crate) struct SearchPath<'a> {
    search_path: &'a [String],
    user: &'a str,
}

impl<'a> SearchPath<'a> {
    /// Return a list of schemas the tables the user can see.
    pub(crate) fn resolve(&'a self) -> Vec<&'a str> {
        let mut schemas = vec![];

        for path in self.search_path {
            match path.as_str() {
                "$user" => schemas.push(self.user),
                path => schemas.push(path),
            }
        }

        schemas
    }

    pub(crate) fn new(
        user: &'a str,
        search_path: Option<&'a ParameterValue>,
        schema: &'a Schema,
    ) -> Self {
        let default_path = schema.search_path();
        let search_path = match search_path {
            Some(ParameterValue::Tuple(overriden)) => overriden.as_slice(),
            Some(ParameterValue::String(overriden)) => std::slice::from_ref(overriden),
            None => default_path,
        };
        Self { search_path, user }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_search_path() {
        let param = vec!["$user".into(), "public".into()];
        let user = "pgdog";
        let resolver = SearchPath {
            search_path: &param,
            user,
        };
        let res = resolver.resolve();
        assert_eq!(res, vec!["pgdog", "public"]);
    }

    #[test]
    fn test_search_path_single_schema_override() {
        let schema = Schema::from_parts(vec!["$user".into(), "public".into()], Default::default());
        // SET search_path TO tenant_b stores a single string, not a tuple.
        let param = ParameterValue::String("tenant_b".into());
        let resolver = SearchPath::new("pgdog", Some(&param), &schema);
        assert_eq!(resolver.resolve(), vec!["tenant_b"]);
    }
}
