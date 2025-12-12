use crate::feat::BASE_URL;
use ekskit::net::{HttpMethod, Request};
use ekskit::prelude::discover;
use ekskit::host::Result;
use ekskit::structs::NovelPreview;

#[discover::get_trending]
pub(crate) fn get_trending() -> Result<Vec<NovelPreview>> {
	let url = format!("{}/", BASE_URL);

	let mut novels: Vec<NovelPreview> = Vec::new();

	let doc = Request::new(&url, HttpMethod::Get).html()?;

	for novel in doc
		.select("#index-novel-hot > div.index-novel > div.item")
		.array()
	{
		let node = novel.as_node().expect("Failed to unwrap novel node");

		novels.push(NovelPreview {
			title: node.select(".title").text().read(),
			cover: node.select("img").attr("src").read(),
			url: format!("{}{}", BASE_URL, node.select("a").attr("href").read()),
		})
	}

	Ok(novels)
}

#[discover::get_most_popular]
pub(crate) fn get_most_popular() -> Result<Vec<NovelPreview>> {
	let url = format!("{}/novel-list/latest-release-novel", BASE_URL);
	fetch_novels(&url)
}

#[discover::get_latest]
pub(crate) fn get_latest() -> Result<Vec<NovelPreview>> {
	let url = format!("{}/novel-list/latest-release-novel", BASE_URL);
	fetch_novels(&url)
}


fn fetch_novels(url: &str) -> Result<Vec<NovelPreview>> {
	let mut novels: Vec<NovelPreview> = Vec::new();

	let doc = Request::new(url, HttpMethod::Get).html()?;

	for novel in doc.select("#list-page .list-novel .row").array() {
		let node = novel.as_node().expect("Failed to unwrap novel node");

		novels.push(NovelPreview {
			title: node.select(".novel-title").text().read(),
			cover: node.select("img.cover").attr("src").read(),
			url: format!("{}{}", BASE_URL, node.select(".novel-title a").attr("href").read()),
		});
	}

	Ok(novels)
}