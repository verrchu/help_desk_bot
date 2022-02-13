use crate::l10n::Lang;

static RU: &str = r#"
_Привет\!_ 😺
*На связи HelpDesk\-бот*
Я знаю, что при переезде в другую страну возникает много вопросов\. Я здесь, чтобы помочь\!

Во мне содержиться очень много полезной информации о переезде в Литву\.🇱🇹 
Когда Вы закончите просматривать информацию, большая просьба вернуться на главное меню, выбрать в самом низу пункт “Опрос” и пройти небольшой опрос\. Ваше мнение поможет мне стать лучше\!  

_Приятного переезда\!_🚚
"#;

pub fn greeting(lang: Lang) -> &'static str {
    match lang {
        Lang::Ru => RU.trim(),
    }
}
