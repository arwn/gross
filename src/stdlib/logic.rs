use crate::{
    json::{new_list, JObject, ToJObject},
    Environment,
};

use super::truthy;

pub fn load_mod(env: &mut Environment) {
    env.insert_builtin("if", |env, args| {
        if args.len() != 3 {
            return new_list(&["error", "bad-arity", &format!("{} != {}", args.len(), 3)]);
        }
        if let &[predicate, t, f] = &args {
            if truthy(&crate::eval(env, predicate)) {
                crate::eval(env, t)
            } else {
                crate::eval(env, f)
            }
        } else {
            JObject::Null
        }
    });

    env.insert_builtin("or", |env, args| {
        for arg in args {
            if truthy(&crate::eval(env, arg)) {
                return true.to_jobject();
            }
        }
        false.to_jobject()
    });
}