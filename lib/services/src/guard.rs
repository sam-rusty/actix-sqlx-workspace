use actix_web::guard::{fn_guard, Guard as ActixGuard};

use crate::middleware::UserClaim;

pub struct Guard;

impl Guard {
    pub fn has_access(_name: String) -> impl ActixGuard + Sized {
        fn_guard(|ctx| {
            let ext = ctx.req_data();
            let _user = ext.get::<UserClaim>();
            true
        })
    }
}
