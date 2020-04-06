use std::path::Path;
use lambda_runtime::{error::HandlerError, lambda, Context};
use serde_json::Value;
use rust_htslib::bam::Reader;

fn main() {
    lambda!(handler)
}

fn handler(
    event: Value,
    _: Context,
) -> Result<Value, HandlerError> {

    let reader = Reader::from_path(&Path::new("/dev/null")).expect("Cannot read BAM file");
    dbg!(reader);
//reader.header().target_names().into_iter()
//        .map(|refname| String::from_utf8_lossy(refname).to_string());

    Ok(event)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn handler_handles() {
        let event = json!({
            "answer": 42
        });
        assert_eq!(
            handler(event.clone(), Context::default()).expect("expected Ok(_) value"),
            event
        )
    }
}
