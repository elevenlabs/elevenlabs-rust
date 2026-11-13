pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum AstNodeOutput {
    #[serde(rename = "add_operator")]
    AddOperator {
        #[serde(flatten)]
        data: AstAdditionOperatorNodeOutput,
    },

    #[serde(rename = "and_operator")]
    AndOperator {
        #[serde(flatten)]
        data: AstAndOperatorNodeOutput,
    },

    #[serde(rename = "boolean_literal")]
    BooleanLiteral {
        #[serde(flatten)]
        data: AstBooleanNodeOutput,
    },

    #[serde(rename = "conditional_operator")]
    ConditionalOperator {
        #[serde(flatten)]
        data: AstConditionalOperatorNodeOutput,
    },

    #[serde(rename = "div_operator")]
    DivOperator {
        #[serde(flatten)]
        data: AstDivisionOperatorNodeOutput,
    },

    #[serde(rename = "dynamic_variable")]
    DynamicVariable {
        #[serde(flatten)]
        data: AstDynamicVariableNodeOutput,
    },

    #[serde(rename = "eq_operator")]
    EqOperator {
        #[serde(flatten)]
        data: AstEqualsOperatorNodeOutput,
    },

    #[serde(rename = "gt_operator")]
    GtOperator {
        #[serde(flatten)]
        data: AstGreaterThanOperatorNodeOutput,
    },

    #[serde(rename = "gte_operator")]
    GteOperator {
        #[serde(flatten)]
        data: AstGreaterThanOrEqualsOperatorNodeOutput,
    },

    #[serde(rename = "llm")]
    Llm {
        #[serde(flatten)]
        data: AstllmNodeOutput,
    },

    #[serde(rename = "lt_operator")]
    LtOperator {
        #[serde(flatten)]
        data: AstLessThanOperatorNodeOutput,
    },

    #[serde(rename = "lte_operator")]
    LteOperator {
        #[serde(flatten)]
        data: AstLessThanOrEqualsOperatorNodeOutput,
    },

    #[serde(rename = "mul_operator")]
    MulOperator {
        #[serde(flatten)]
        data: AstMultiplicationOperatorNodeOutput,
    },

    #[serde(rename = "neq_operator")]
    NeqOperator {
        #[serde(flatten)]
        data: AstNotEqualsOperatorNodeOutput,
    },

    #[serde(rename = "null_literal")]
    NullLiteral {
        #[serde(flatten)]
        data: AstNullNodeOutput,
    },

    #[serde(rename = "number_literal")]
    NumberLiteral {
        #[serde(flatten)]
        data: AstNumberNodeOutput,
    },

    #[serde(rename = "or_operator")]
    OrOperator {
        #[serde(flatten)]
        data: AstOrOperatorNodeOutput,
    },

    #[serde(rename = "string_literal")]
    StringLiteral {
        #[serde(flatten)]
        data: AstStringNodeOutput,
    },

    #[serde(rename = "sub_operator")]
    SubOperator {
        #[serde(flatten)]
        data: AstSubtractionOperatorNodeOutput,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl AstNodeOutput {
    pub fn is_add_operator(&self) -> bool {
        matches!(self, Self::AddOperator { .. })
    }

    pub fn as_add_operator(&self) -> Option<&AstAdditionOperatorNodeOutput> {
        match self {
            Self::AddOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_add_operator(self) -> Option<AstAdditionOperatorNodeOutput> {
        match self {
            Self::AddOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_and_operator(&self) -> bool {
        matches!(self, Self::AndOperator { .. })
    }

    pub fn as_and_operator(&self) -> Option<&AstAndOperatorNodeOutput> {
        match self {
            Self::AndOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_and_operator(self) -> Option<AstAndOperatorNodeOutput> {
        match self {
            Self::AndOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_boolean_literal(&self) -> bool {
        matches!(self, Self::BooleanLiteral { .. })
    }

    pub fn as_boolean_literal(&self) -> Option<&AstBooleanNodeOutput> {
        match self {
            Self::BooleanLiteral { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_boolean_literal(self) -> Option<AstBooleanNodeOutput> {
        match self {
            Self::BooleanLiteral { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_conditional_operator(&self) -> bool {
        matches!(self, Self::ConditionalOperator { .. })
    }

    pub fn as_conditional_operator(&self) -> Option<&AstConditionalOperatorNodeOutput> {
        match self {
            Self::ConditionalOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_conditional_operator(self) -> Option<AstConditionalOperatorNodeOutput> {
        match self {
            Self::ConditionalOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_div_operator(&self) -> bool {
        matches!(self, Self::DivOperator { .. })
    }

    pub fn as_div_operator(&self) -> Option<&AstDivisionOperatorNodeOutput> {
        match self {
            Self::DivOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_div_operator(self) -> Option<AstDivisionOperatorNodeOutput> {
        match self {
            Self::DivOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_dynamic_variable(&self) -> bool {
        matches!(self, Self::DynamicVariable { .. })
    }

    pub fn as_dynamic_variable(&self) -> Option<&AstDynamicVariableNodeOutput> {
        match self {
            Self::DynamicVariable { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_dynamic_variable(self) -> Option<AstDynamicVariableNodeOutput> {
        match self {
            Self::DynamicVariable { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_eq_operator(&self) -> bool {
        matches!(self, Self::EqOperator { .. })
    }

    pub fn as_eq_operator(&self) -> Option<&AstEqualsOperatorNodeOutput> {
        match self {
            Self::EqOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_eq_operator(self) -> Option<AstEqualsOperatorNodeOutput> {
        match self {
            Self::EqOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_gt_operator(&self) -> bool {
        matches!(self, Self::GtOperator { .. })
    }

    pub fn as_gt_operator(&self) -> Option<&AstGreaterThanOperatorNodeOutput> {
        match self {
            Self::GtOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_gt_operator(self) -> Option<AstGreaterThanOperatorNodeOutput> {
        match self {
            Self::GtOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_gte_operator(&self) -> bool {
        matches!(self, Self::GteOperator { .. })
    }

    pub fn as_gte_operator(&self) -> Option<&AstGreaterThanOrEqualsOperatorNodeOutput> {
        match self {
            Self::GteOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_gte_operator(self) -> Option<AstGreaterThanOrEqualsOperatorNodeOutput> {
        match self {
            Self::GteOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_llm(&self) -> bool {
        matches!(self, Self::Llm { .. })
    }

    pub fn as_llm(&self) -> Option<&AstllmNodeOutput> {
        match self {
            Self::Llm { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_llm(self) -> Option<AstllmNodeOutput> {
        match self {
            Self::Llm { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_lt_operator(&self) -> bool {
        matches!(self, Self::LtOperator { .. })
    }

    pub fn as_lt_operator(&self) -> Option<&AstLessThanOperatorNodeOutput> {
        match self {
            Self::LtOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_lt_operator(self) -> Option<AstLessThanOperatorNodeOutput> {
        match self {
            Self::LtOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_lte_operator(&self) -> bool {
        matches!(self, Self::LteOperator { .. })
    }

    pub fn as_lte_operator(&self) -> Option<&AstLessThanOrEqualsOperatorNodeOutput> {
        match self {
            Self::LteOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_lte_operator(self) -> Option<AstLessThanOrEqualsOperatorNodeOutput> {
        match self {
            Self::LteOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_mul_operator(&self) -> bool {
        matches!(self, Self::MulOperator { .. })
    }

    pub fn as_mul_operator(&self) -> Option<&AstMultiplicationOperatorNodeOutput> {
        match self {
            Self::MulOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_mul_operator(self) -> Option<AstMultiplicationOperatorNodeOutput> {
        match self {
            Self::MulOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_neq_operator(&self) -> bool {
        matches!(self, Self::NeqOperator { .. })
    }

    pub fn as_neq_operator(&self) -> Option<&AstNotEqualsOperatorNodeOutput> {
        match self {
            Self::NeqOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_neq_operator(self) -> Option<AstNotEqualsOperatorNodeOutput> {
        match self {
            Self::NeqOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_null_literal(&self) -> bool {
        matches!(self, Self::NullLiteral { .. })
    }

    pub fn as_null_literal(&self) -> Option<&AstNullNodeOutput> {
        match self {
            Self::NullLiteral { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_null_literal(self) -> Option<AstNullNodeOutput> {
        match self {
            Self::NullLiteral { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_number_literal(&self) -> bool {
        matches!(self, Self::NumberLiteral { .. })
    }

    pub fn as_number_literal(&self) -> Option<&AstNumberNodeOutput> {
        match self {
            Self::NumberLiteral { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_number_literal(self) -> Option<AstNumberNodeOutput> {
        match self {
            Self::NumberLiteral { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_or_operator(&self) -> bool {
        matches!(self, Self::OrOperator { .. })
    }

    pub fn as_or_operator(&self) -> Option<&AstOrOperatorNodeOutput> {
        match self {
            Self::OrOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_or_operator(self) -> Option<AstOrOperatorNodeOutput> {
        match self {
            Self::OrOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_string_literal(&self) -> bool {
        matches!(self, Self::StringLiteral { .. })
    }

    pub fn as_string_literal(&self) -> Option<&AstStringNodeOutput> {
        match self {
            Self::StringLiteral { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_string_literal(self) -> Option<AstStringNodeOutput> {
        match self {
            Self::StringLiteral { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_sub_operator(&self) -> bool {
        matches!(self, Self::SubOperator { .. })
    }

    pub fn as_sub_operator(&self) -> Option<&AstSubtractionOperatorNodeOutput> {
        match self {
            Self::SubOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_sub_operator(self) -> Option<AstSubtractionOperatorNodeOutput> {
        match self {
            Self::SubOperator { data } => Some(data),
            _ => None,
        }
    }
}
