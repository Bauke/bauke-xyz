use color_eyre::Result;

/**
Minify HTML using [`minify-html`].
*/
pub fn html(data: String) -> Result<String> {
  let minify_config = minify_html::Cfg {
    do_not_minify_doctype: true,
    ensure_spec_compliant_unquoted_attribute_values: true,
    keep_closing_tags: true,
    keep_comments: false,
    keep_html_and_head_opening_tags: true,
    keep_spaces_between_attributes: true,
    minify_css: false,
    minify_js: false,
    remove_bangs: false,
    remove_processing_instructions: false,
  };

  let minified_data = minify_html::minify(&data.into_bytes(), &minify_config);
  String::from_utf8(minified_data).map_err(Into::into)
}
