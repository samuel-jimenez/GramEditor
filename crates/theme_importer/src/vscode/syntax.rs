use indexmap::IndexMap;
use serde::Deserialize;
use strum::EnumIter;

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum VsCodeTokenScope {
    One(String),
    Many(Vec<String>),
}

#[derive(Debug, Deserialize)]
pub struct VsCodeTokenColor {
    pub name: Option<String>,
    pub scope: Option<VsCodeTokenScope>,
    pub settings: VsCodeTokenColorSettings,
}

#[derive(Debug, Deserialize)]
pub struct VsCodeTokenColorSettings {
    pub foreground: Option<String>,
    pub background: Option<String>,
    #[serde(rename = "fontStyle")]
    pub font_style: Option<String>,
}

#[derive(Debug, PartialEq, Copy, Clone, EnumIter)]
pub enum TehanuSyntaxToken {
    Attribute,
    Boolean,
    Comment,
    CommentDoc,
    Constant,
    Constructor,
    Embedded,
    Emphasis,
    EmphasisStrong,
    Enum,
    Function,
    Hint,
    Keyword,
    Label,
    LinkText,
    LinkUri,
    Number,
    Operator,
    Predictive,
    Preproc,
    Primary,
    Property,
    Punctuation,
    PunctuationBracket,
    PunctuationDelimiter,
    PunctuationListMarker,
    PunctuationSpecial,
    String,
    StringEscape,
    StringRegex,
    StringSpecial,
    StringSpecialSymbol,
    Tag,
    TextLiteral,
    Title,
    Type,
    Variable,
    VariableSpecial,
    Variant,
}

impl std::fmt::Display for TehanuSyntaxToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TehanuSyntaxToken::Attribute => "attribute",
                TehanuSyntaxToken::Boolean => "boolean",
                TehanuSyntaxToken::Comment => "comment",
                TehanuSyntaxToken::CommentDoc => "comment.doc",
                TehanuSyntaxToken::Constant => "constant",
                TehanuSyntaxToken::Constructor => "constructor",
                TehanuSyntaxToken::Embedded => "embedded",
                TehanuSyntaxToken::Emphasis => "emphasis",
                TehanuSyntaxToken::EmphasisStrong => "emphasis.strong",
                TehanuSyntaxToken::Enum => "enum",
                TehanuSyntaxToken::Function => "function",
                TehanuSyntaxToken::Hint => "hint",
                TehanuSyntaxToken::Keyword => "keyword",
                TehanuSyntaxToken::Label => "label",
                TehanuSyntaxToken::LinkText => "link_text",
                TehanuSyntaxToken::LinkUri => "link_uri",
                TehanuSyntaxToken::Number => "number",
                TehanuSyntaxToken::Operator => "operator",
                TehanuSyntaxToken::Predictive => "predictive",
                TehanuSyntaxToken::Preproc => "preproc",
                TehanuSyntaxToken::Primary => "primary",
                TehanuSyntaxToken::Property => "property",
                TehanuSyntaxToken::Punctuation => "punctuation",
                TehanuSyntaxToken::PunctuationBracket => "punctuation.bracket",
                TehanuSyntaxToken::PunctuationDelimiter => "punctuation.delimiter",
                TehanuSyntaxToken::PunctuationListMarker => "punctuation.list_marker",
                TehanuSyntaxToken::PunctuationSpecial => "punctuation.special",
                TehanuSyntaxToken::String => "string",
                TehanuSyntaxToken::StringEscape => "string.escape",
                TehanuSyntaxToken::StringRegex => "string.regex",
                TehanuSyntaxToken::StringSpecial => "string.special",
                TehanuSyntaxToken::StringSpecialSymbol => "string.special.symbol",
                TehanuSyntaxToken::Tag => "tag",
                TehanuSyntaxToken::TextLiteral => "text.literal",
                TehanuSyntaxToken::Title => "title",
                TehanuSyntaxToken::Type => "type",
                TehanuSyntaxToken::Variable => "variable",
                TehanuSyntaxToken::VariableSpecial => "variable.special",
                TehanuSyntaxToken::Variant => "variant",
            }
        )
    }
}

impl TehanuSyntaxToken {
    pub fn find_best_token_color_match<'a>(
        &self,
        token_colors: &'a [VsCodeTokenColor],
    ) -> Option<&'a VsCodeTokenColor> {
        let mut ranked_matches = IndexMap::new();

        for (ix, token_color) in token_colors.iter().enumerate() {
            if token_color.settings.foreground.is_none() {
                continue;
            }

            let Some(rank) = self.rank_match(token_color) else {
                continue;
            };

            if rank > 0 {
                ranked_matches.insert(ix, rank);
            }
        }

        ranked_matches
            .into_iter()
            .max_by_key(|(_, rank)| *rank)
            .map(|(ix, _)| &token_colors[ix])
    }

    fn rank_match(&self, token_color: &VsCodeTokenColor) -> Option<u32> {
        let candidate_scopes = match token_color.scope.as_ref()? {
            VsCodeTokenScope::One(scope) => vec![scope],
            VsCodeTokenScope::Many(scopes) => scopes.iter().collect(),
        }
        .iter()
        .flat_map(|scope| scope.split(',').map(|s| s.trim()))
        .collect::<Vec<_>>();

        let scopes_to_match = self.to_vscode();
        let number_of_scopes_to_match = scopes_to_match.len();

        let mut matches = 0;

        for (ix, scope) in scopes_to_match.into_iter().enumerate() {
            // Assign each entry a weight that is inversely proportional to its
            // position in the list.
            //
            // Entries towards the front are weighted higher than those towards the end.
            let weight = (number_of_scopes_to_match - ix) as u32;

            if candidate_scopes.contains(&scope) {
                matches += 1 + weight;
            }
        }

        Some(matches)
    }

    pub fn fallbacks(&self) -> &[Self] {
        match self {
            TehanuSyntaxToken::CommentDoc => &[TehanuSyntaxToken::Comment],
            TehanuSyntaxToken::Number => &[TehanuSyntaxToken::Constant],
            TehanuSyntaxToken::VariableSpecial => &[TehanuSyntaxToken::Variable],
            TehanuSyntaxToken::PunctuationBracket
            | TehanuSyntaxToken::PunctuationDelimiter
            | TehanuSyntaxToken::PunctuationListMarker
            | TehanuSyntaxToken::PunctuationSpecial => &[TehanuSyntaxToken::Punctuation],
            TehanuSyntaxToken::StringEscape
            | TehanuSyntaxToken::StringRegex
            | TehanuSyntaxToken::StringSpecial
            | TehanuSyntaxToken::StringSpecialSymbol => &[TehanuSyntaxToken::String],
            _ => &[],
        }
    }

    fn to_vscode(self) -> Vec<&'static str> {
        match self {
            TehanuSyntaxToken::Attribute => vec!["entity.other.attribute-name"],
            TehanuSyntaxToken::Boolean => vec!["constant.language"],
            TehanuSyntaxToken::Comment => vec!["comment"],
            TehanuSyntaxToken::CommentDoc => vec!["comment.block.documentation"],
            TehanuSyntaxToken::Constant => vec!["constant", "constant.language", "constant.character"],
            TehanuSyntaxToken::Constructor => {
                vec![
                    "entity.name.tag",
                    "entity.name.function.definition.special.constructor",
                ]
            }
            TehanuSyntaxToken::Embedded => vec!["meta.embedded"],
            TehanuSyntaxToken::Emphasis => vec!["markup.italic"],
            TehanuSyntaxToken::EmphasisStrong => vec![
                "markup.bold",
                "markup.italic markup.bold",
                "markup.bold markup.italic",
            ],
            TehanuSyntaxToken::Enum => vec!["support.type.enum"],
            TehanuSyntaxToken::Function => vec![
                "entity.function",
                "entity.name.function",
                "variable.function",
            ],
            TehanuSyntaxToken::Hint => vec![],
            TehanuSyntaxToken::Keyword => vec![
                "keyword",
                "keyword.other.fn.rust",
                "keyword.control",
                "keyword.control.fun",
                "keyword.control.class",
                "punctuation.accessor",
                "entity.name.tag",
            ],
            TehanuSyntaxToken::Label => vec![
                "label",
                "entity.name",
                "entity.name.import",
                "entity.name.package",
            ],
            TehanuSyntaxToken::LinkText => vec!["markup.underline.link", "string.other.link"],
            TehanuSyntaxToken::LinkUri => vec!["markup.underline.link", "string.other.link"],
            TehanuSyntaxToken::Number => vec!["constant.numeric", "number"],
            TehanuSyntaxToken::Operator => vec!["operator", "keyword.operator"],
            TehanuSyntaxToken::Predictive => vec![],
            TehanuSyntaxToken::Preproc => vec![
                "preproc",
                "meta.preprocessor",
                "punctuation.definition.preprocessor",
            ],
            TehanuSyntaxToken::Primary => vec![],
            TehanuSyntaxToken::Property => vec![
                "variable.member",
                "support.type.property-name",
                "variable.object.property",
                "variable.other.field",
            ],
            TehanuSyntaxToken::Punctuation => vec![
                "punctuation",
                "punctuation.section",
                "punctuation.accessor",
                "punctuation.separator",
                "punctuation.definition.tag",
            ],
            TehanuSyntaxToken::PunctuationBracket => vec![
                "punctuation.bracket",
                "punctuation.definition.tag.begin",
                "punctuation.definition.tag.end",
            ],
            TehanuSyntaxToken::PunctuationDelimiter => vec![
                "punctuation.delimiter",
                "punctuation.separator",
                "punctuation.terminator",
            ],
            TehanuSyntaxToken::PunctuationListMarker => {
                vec!["markup.list punctuation.definition.list.begin"]
            }
            TehanuSyntaxToken::PunctuationSpecial => vec!["punctuation.special"],
            TehanuSyntaxToken::String => vec!["string"],
            TehanuSyntaxToken::StringEscape => {
                vec!["string.escape", "constant.character", "constant.other"]
            }
            TehanuSyntaxToken::StringRegex => vec!["string.regex"],
            TehanuSyntaxToken::StringSpecial => vec!["string.special", "constant.other.symbol"],
            TehanuSyntaxToken::StringSpecialSymbol => {
                vec!["string.special.symbol", "constant.other.symbol"]
            }
            TehanuSyntaxToken::Tag => vec!["tag", "entity.name.tag", "meta.tag.sgml"],
            TehanuSyntaxToken::TextLiteral => vec!["text.literal", "string"],
            TehanuSyntaxToken::Title => vec!["title", "entity.name"],
            TehanuSyntaxToken::Type => vec![
                "entity.name.type",
                "entity.name.type.primitive",
                "entity.name.type.numeric",
                "keyword.type",
                "support.type",
                "support.type.primitive",
                "support.class",
            ],
            TehanuSyntaxToken::Variable => vec![
                "variable",
                "variable.language",
                "variable.member",
                "variable.parameter",
                "variable.parameter.function-call",
            ],
            TehanuSyntaxToken::VariableSpecial => vec![
                "variable.special",
                "variable.member",
                "variable.annotation",
                "variable.language",
            ],
            TehanuSyntaxToken::Variant => vec!["variant"],
        }
    }
}
