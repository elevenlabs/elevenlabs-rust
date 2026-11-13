pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum AstNodeInput {
    #[serde(rename = "add_operator")]
    AddOperator {
        #[serde(flatten)]
        data: AstAdditionOperatorNodeInput,
    },

    #[serde(rename = "and_operator")]
    AndOperator {
        #[serde(flatten)]
        data: AstAndOperatorNodeInput,
    },

    #[serde(rename = "boolean_literal")]
    BooleanLiteral {
        #[serde(flatten)]
        data: AstBooleanNodeInput,
    },

    #[serde(rename = "conditional_operator")]
    ConditionalOperator {
        #[serde(flatten)]
        data: AstConditionalOperatorNodeInput,
    },

    #[serde(rename = "div_operator")]
    DivOperator {
        #[serde(flatten)]
        data: AstDivisionOperatorNodeInput,
    },

    #[serde(rename = "dynamic_variable")]
    DynamicVariable {
        #[serde(flatten)]
        data: AstDynamicVariableNodeInput,
    },

    #[serde(rename = "eq_operator")]
    EqOperator {
        #[serde(flatten)]
        data: AstEqualsOperatorNodeInput,
    },

    #[serde(rename = "gt_operator")]
    GtOperator {
        #[serde(flatten)]
        data: AstGreaterThanOperatorNodeInput,
    },

    #[serde(rename = "gte_operator")]
    GteOperator {
        #[serde(flatten)]
        data: AstGreaterThanOrEqualsOperatorNodeInput,
    },

    #[serde(rename = "llm")]
    Llm {
        #[serde(flatten)]
        data: AstllmNodeInput,
    },

    #[serde(rename = "lt_operator")]
    LtOperator {
        #[serde(flatten)]
        data: AstLessThanOperatorNodeInput,
    },

    #[serde(rename = "lte_operator")]
    LteOperator {
        #[serde(flatten)]
        data: AstLessThanOrEqualsOperatorNodeInput,
    },

    #[serde(rename = "mul_operator")]
    MulOperator {
        #[serde(flatten)]
        data: AstMultiplicationOperatorNodeInput,
    },

    #[serde(rename = "neq_operator")]
    NeqOperator {
        #[serde(flatten)]
        data: AstNotEqualsOperatorNodeInput,
    },

    #[serde(rename = "null_literal")]
    NullLiteral {
        #[serde(flatten)]
        data: AstNullNodeInput,
    },

    #[serde(rename = "number_literal")]
    NumberLiteral {
        #[serde(flatten)]
        data: AstNumberNodeInput,
    },

    #[serde(rename = "or_operator")]
    OrOperator {
        #[serde(flatten)]
        data: AstOrOperatorNodeInput,
    },

    #[serde(rename = "string_literal")]
    StringLiteral {
        #[serde(flatten)]
        data: AstStringNodeInput,
    },

    #[serde(rename = "sub_operator")]
    SubOperator {
        #[serde(flatten)]
        data: AstSubtractionOperatorNodeInput,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl AstNodeInput {
    pub fn is_add_operator(&self) -> bool {
        matches!(self, Self::AddOperator { .. })
    }

    pub fn as_add_operator(&self) -> Option<&AstAdditionOperatorNodeInput> {
        match self {
            Self::AddOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_add_operator(self) -> Option<AstAdditionOperatorNodeInput> {
        match self {
            Self::AddOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_and_operator(&self) -> bool {
        matches!(self, Self::AndOperator { .. })
    }

    pub fn as_and_operator(&self) -> Option<&AstAndOperatorNodeInput> {
        match self {
            Self::AndOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_and_operator(self) -> Option<AstAndOperatorNodeInput> {
        match self {
            Self::AndOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_boolean_literal(&self) -> bool {
        matches!(self, Self::BooleanLiteral { .. })
    }

    pub fn as_boolean_literal(&self) -> Option<&AstBooleanNodeInput> {
        match self {
            Self::BooleanLiteral { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_boolean_literal(self) -> Option<AstBooleanNodeInput> {
        match self {
            Self::BooleanLiteral { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_conditional_operator(&self) -> bool {
        matches!(self, Self::ConditionalOperator { .. })
    }

    pub fn as_conditional_operator(&self) -> Option<&AstConditionalOperatorNodeInput> {
        match self {
            Self::ConditionalOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_conditional_operator(self) -> Option<AstConditionalOperatorNodeInput> {
        match self {
            Self::ConditionalOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_div_operator(&self) -> bool {
        matches!(self, Self::DivOperator { .. })
    }

    pub fn as_div_operator(&self) -> Option<&AstDivisionOperatorNodeInput> {
        match self {
            Self::DivOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_div_operator(self) -> Option<AstDivisionOperatorNodeInput> {
        match self {
            Self::DivOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_dynamic_variable(&self) -> bool {
        matches!(self, Self::DynamicVariable { .. })
    }

    pub fn as_dynamic_variable(&self) -> Option<&AstDynamicVariableNodeInput> {
        match self {
            Self::DynamicVariable { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_dynamic_variable(self) -> Option<AstDynamicVariableNodeInput> {
        match self {
            Self::DynamicVariable { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_eq_operator(&self) -> bool {
        matches!(self, Self::EqOperator { .. })
    }

    pub fn as_eq_operator(&self) -> Option<&AstEqualsOperatorNodeInput> {
        match self {
            Self::EqOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_eq_operator(self) -> Option<AstEqualsOperatorNodeInput> {
        match self {
            Self::EqOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_gt_operator(&self) -> bool {
        matches!(self, Self::GtOperator { .. })
    }

    pub fn as_gt_operator(&self) -> Option<&AstGreaterThanOperatorNodeInput> {
        match self {
            Self::GtOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_gt_operator(self) -> Option<AstGreaterThanOperatorNodeInput> {
        match self {
            Self::GtOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_gte_operator(&self) -> bool {
        matches!(self, Self::GteOperator { .. })
    }

    pub fn as_gte_operator(&self) -> Option<&AstGreaterThanOrEqualsOperatorNodeInput> {
        match self {
            Self::GteOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_gte_operator(self) -> Option<AstGreaterThanOrEqualsOperatorNodeInput> {
        match self {
            Self::GteOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_llm(&self) -> bool {
        matches!(self, Self::Llm { .. })
    }

    pub fn as_llm(&self) -> Option<&AstllmNodeInput> {
        match self {
            Self::Llm { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_llm(self) -> Option<AstllmNodeInput> {
        match self {
            Self::Llm { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_lt_operator(&self) -> bool {
        matches!(self, Self::LtOperator { .. })
    }

    pub fn as_lt_operator(&self) -> Option<&AstLessThanOperatorNodeInput> {
        match self {
            Self::LtOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_lt_operator(self) -> Option<AstLessThanOperatorNodeInput> {
        match self {
            Self::LtOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_lte_operator(&self) -> bool {
        matches!(self, Self::LteOperator { .. })
    }

    pub fn as_lte_operator(&self) -> Option<&AstLessThanOrEqualsOperatorNodeInput> {
        match self {
            Self::LteOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_lte_operator(self) -> Option<AstLessThanOrEqualsOperatorNodeInput> {
        match self {
            Self::LteOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_mul_operator(&self) -> bool {
        matches!(self, Self::MulOperator { .. })
    }

    pub fn as_mul_operator(&self) -> Option<&AstMultiplicationOperatorNodeInput> {
        match self {
            Self::MulOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_mul_operator(self) -> Option<AstMultiplicationOperatorNodeInput> {
        match self {
            Self::MulOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_neq_operator(&self) -> bool {
        matches!(self, Self::NeqOperator { .. })
    }

    pub fn as_neq_operator(&self) -> Option<&AstNotEqualsOperatorNodeInput> {
        match self {
            Self::NeqOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_neq_operator(self) -> Option<AstNotEqualsOperatorNodeInput> {
        match self {
            Self::NeqOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_null_literal(&self) -> bool {
        matches!(self, Self::NullLiteral { .. })
    }

    pub fn as_null_literal(&self) -> Option<&AstNullNodeInput> {
        match self {
            Self::NullLiteral { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_null_literal(self) -> Option<AstNullNodeInput> {
        match self {
            Self::NullLiteral { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_number_literal(&self) -> bool {
        matches!(self, Self::NumberLiteral { .. })
    }

    pub fn as_number_literal(&self) -> Option<&AstNumberNodeInput> {
        match self {
            Self::NumberLiteral { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_number_literal(self) -> Option<AstNumberNodeInput> {
        match self {
            Self::NumberLiteral { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_or_operator(&self) -> bool {
        matches!(self, Self::OrOperator { .. })
    }

    pub fn as_or_operator(&self) -> Option<&AstOrOperatorNodeInput> {
        match self {
            Self::OrOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_or_operator(self) -> Option<AstOrOperatorNodeInput> {
        match self {
            Self::OrOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_string_literal(&self) -> bool {
        matches!(self, Self::StringLiteral { .. })
    }

    pub fn as_string_literal(&self) -> Option<&AstStringNodeInput> {
        match self {
            Self::StringLiteral { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_string_literal(self) -> Option<AstStringNodeInput> {
        match self {
            Self::StringLiteral { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_sub_operator(&self) -> bool {
        matches!(self, Self::SubOperator { .. })
    }

    pub fn as_sub_operator(&self) -> Option<&AstSubtractionOperatorNodeInput> {
        match self {
            Self::SubOperator { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_sub_operator(self) -> Option<AstSubtractionOperatorNodeInput> {
        match self {
            Self::SubOperator { data } => Some(data),
            _ => None,
        }
    }
}
