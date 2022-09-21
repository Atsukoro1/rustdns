use reqwest::get;

/// Returns sorted Vector of TLDs
pub async fn fp_tlds() -> Result<Vec<String>, reqwest::Error> {
    let text_res: String = get("https://data.iana.org/TLD/tlds-alpha-by-domain.txt")
        .await?
        .text()
        .await?;

    let mut split_r: Vec<String> = text_res.split("\n")
        .into_iter()
        .map(|item: &str| {
            item.to_string()
        })
        .collect::<Vec<String>>();

    // Remove the version and date info and the last EOF byte
    split_r.remove(0);
    split_r.remove(split_r.len() - 1);

    /*
        Sort the list alphabetical, so binary search can be used 
        with this vector
    */
    split_r.sort_by(|st, nd| {
        return st.partial_cmp(nd).unwrap()
    });

    Ok(
        split_r
    )
}