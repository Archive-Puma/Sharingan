use ring::digest::{Context,Digest,SHA256};

pub fn hash(password: &str) -> Digest {
    let mut ctx = Context::new(&SHA256);
    ctx.update(password.as_bytes());
    ctx.finish()
}