#[derive(Debug, Clone)]
pub struct CommonThemes {
    pub colors: Colors,
    pub symbols: Symbols,
    pub emphasis: Emphasis,
}

#[derive(Debug, Clone)]
pub struct Colors {
    pub primary: ColorToken,
    pub secondary: ColorToken,
    pub success: ColorToken,
    pub warning: ColorToken,
    pub error: ColorToken,
    pub muted: ColorToken,
}

#[derive(Debug, Clone)]
pub struct Symbols {
    pub info: char,
    pub success: char,
    pub warning: char,
    pub error: char,
    pub arrow: char,
}

#[derive(Debug, Clone)]
pub struct Emphasis {
    pub strong: TextStyles,
    pub subtle: TextStyles,
}
#[derive(Debug, Clone)]
struct TextStyles{
    pub weight: TextWeight,
    pub intensity: TextIntensity,   
}

#[derive(Debug, Clone)]
pub enum TextWeight {
    Normal,
    Strong,
}

#[derive(Debug, Clone)]
pub enum TextIntensity {
    Normal,
    Subtle,
}
#[derive(Debug, Clone)]
pub enum ColorToken {
    Primary,
    Secondary,
    Success,
    Warning,
    Error,
    Muted,
}

impl Default for Emphasis {
    fn default() -> Self {
        Self {
            strong: TextStyles {
                weight: TextWeight::Strong,
                intensity: TextIntensity::Normal,
            },
            subtle: TextStyles{
                weight: TextWeight::Normal,
                intensity: TextIntensity::Subtle,
            },
            
            normal: TextStyles {
                weight: TextWeight::Normal,
                intensity: TextIntensity::Normal,
            },
        }
    }
}
