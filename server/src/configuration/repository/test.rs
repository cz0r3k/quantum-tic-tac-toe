use super::*;
#[test]
fn test_from_str() {
    assert_eq!(
        <Repository as FromStr>::from_str("local"),
        Ok(Repository::Local)
    );
    assert_eq!(
        <Repository as FromStr>::from_str("LOCAL"),
        Ok(Repository::Local)
    );
    assert_eq!(
        <Repository as FromStr>::from_str("Local"),
        Ok(Repository::Local)
    );
    assert_eq!(
        <Repository as FromStr>::from_str("LoCaL"),
        Ok(Repository::Local)
    );
    assert_eq!(<Repository as FromStr>::from_str("LOC"), Err(()));

    assert_eq!(
        <Repository as FromStr>::from_str("REDIS"),
        Ok(Repository::Redis)
    );
    assert_eq!(
        <Repository as FromStr>::from_str("redis"),
        Ok(Repository::Redis)
    );
    assert_eq!(
        <Repository as FromStr>::from_str("Redis"),
        Ok(Repository::Redis)
    );
    assert_eq!(
        <Repository as FromStr>::from_str("ReDiS"),
        Ok(Repository::Redis)
    );
    assert_eq!(<Repository as FromStr>::from_str("RED"), Err(()));
}
