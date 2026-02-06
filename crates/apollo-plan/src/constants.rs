use apollo_utils::pl_str::PlSmallStr;

pub const CSE_REPLACED: &str = "__APOLLO_CSER_";
pub const APOLLO_TMP_PREFIX: &str = "_APOLLO_";
pub const APOLLO_PLACEHOLDER: &str = "_APOLLO_<>";
pub const APOLLO_ELEMENT: &str = "__PL_ELEMENT";
pub const APOLLO_STRUCTFIELDS: &str = "__PL_STRUCTFIELDS";
pub const LEN: &str = "len";

const LITERAL_NAME: PlSmallStr = PlSmallStr::from_static("literal");
const LEN_NAME: PlSmallStr = PlSmallStr::from_static(LEN);
const PL_ELEMENT_NAME: PlSmallStr = PlSmallStr::from_static(APOLLO_ELEMENT);
const PL_STRUCTFIELDS_NAME: PlSmallStr = PlSmallStr::from_static(APOLLO_STRUCTFIELDS);

pub fn get_literal_name() -> PlSmallStr {
    LITERAL_NAME.clone()
}

pub(crate) fn get_len_name() -> PlSmallStr {
    LEN_NAME.clone()
}

pub fn get_pl_element_name() -> PlSmallStr {
    PL_ELEMENT_NAME.clone()
}

pub fn get_pl_structfields_name() -> PlSmallStr {
    PL_STRUCTFIELDS_NAME.clone()
}
