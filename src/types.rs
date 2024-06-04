use serde_json::Value;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Agent {
	uid: Arc<str>,
	description: Option<String>,
	instruction: Option<String>,
	tools: Option<Tools>,
	// -- Future
	// Prompt template are not supported yet
	// prompt_template: Option<PromptTemplate>,
}

// region:    --- Tools

// NOTE: Tool types might come from `genai` library.
//       Or might have some from<..> implementations

#[derive(Debug, Clone)]
pub struct Tools {}

#[derive(Debug, Clone)]
pub struct Tool {
	kind: TookKind,
	name: String,
	instruction: Option<String>,
	description: Option<String>,
	spec: Value,
}

#[derive(Debug, Clone)]
pub enum TookKind {
	Function,
}

// endregion: --- Tools

// region:    --- Template

// NOTE: Here it is just to show the possible direction.
//       This is not fully designed yet.

#[derive(Debug, Clone)]
pub struct PromptTemplate {
	kind: TemplateKind,
	content: String,
}

#[derive(Debug, Clone)]
pub enum TemplateKind {
	Handlebars, // will be the first
	Lua,        // in a far future
}

// endregion: --- Template
