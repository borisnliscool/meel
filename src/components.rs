use std::collections::HashMap;

pub fn apply_components(content: String, data: HashMap<String, String>) -> Result<String, String> {
    // TODO: Loop over all the html elements inside the content and check if they start with 'meel'
    //  if so, we assume that it's a component and render it. Use it's data-attributes to render the
    //  placeholders inside the component.
    //  Example usage: <meel-button href="{{ url }}">Hello</meel-button>
    

    Ok(content)
}