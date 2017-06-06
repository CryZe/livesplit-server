use livesplit_core::Timer;
use livesplit_core::component::{title, splits, timer, previous_segment, possible_time_save,
                                sum_of_best, graph, text, total_playtime, current_pace};

#[derive(Deserialize)]
pub struct LayoutSettings(pub Vec<ComponentSettings>);

pub enum Component {
    Title(title::Component),
    Splits(splits::Component),
    Timer(timer::Component),
    PreviousSegment(previous_segment::Component),
    PossibleTimeSave(possible_time_save::Component),
    SumOfBest(sum_of_best::Component),
    Graph(graph::Component),
    Text(text::Component),
    TotalPlaytime(total_playtime::Component),
    CurrentPace(current_pace::Component),
}

#[derive(Serialize)]
pub enum ComponentState {
    Title(title::State),
    Splits(splits::State),
    Timer(timer::State),
    PreviousSegment(previous_segment::State),
    PossibleTimeSave(possible_time_save::State),
    SumOfBest(sum_of_best::State),
    Graph(graph::State),
    Text(text::State),
    TotalPlaytime(total_playtime::State),
    CurrentPace(current_pace::State),
}

#[derive(Serialize, Deserialize)]
pub enum ComponentSettings {
    Title,
    Splits(splits::Settings),
    Timer,
    PreviousSegment,
    PossibleTimeSave,
    SumOfBest,
    Graph(graph::Settings),
    Text(text::Settings),
    TotalPlaytime,
    CurrentPace(current_pace::Settings),
}

impl From<ComponentSettings> for Component {
    fn from(settings: ComponentSettings) -> Self {
        match settings {
            ComponentSettings::Title => Component::Title(title::Component::new()),
            ComponentSettings::Splits(settings) => {
                let mut component = splits::Component::new();
                *component.settings_mut() = settings;
                Component::Splits(component)
            }
            ComponentSettings::Timer => Component::Timer(timer::Component::new()),
            ComponentSettings::PreviousSegment => {
                Component::PreviousSegment(previous_segment::Component::new())
            }
            ComponentSettings::PossibleTimeSave => {
                Component::PossibleTimeSave(possible_time_save::Component::new())
            }
            ComponentSettings::SumOfBest => Component::SumOfBest(sum_of_best::Component::new()),
            ComponentSettings::Graph(settings) => {
                let mut component = graph::Component::new();
                *component.settings_mut() = settings;
                Component::Graph(component)
            }
            ComponentSettings::Text(settings) => {
                let mut component = text::Component::new();
                *component.settings_mut() = settings;
                Component::Text(component)
            }
            ComponentSettings::TotalPlaytime => {
                Component::TotalPlaytime(total_playtime::Component::new())
            }
            ComponentSettings::CurrentPace(settings) => {
                let mut component = current_pace::Component::new();
                *component.settings_mut() = settings;
                Component::CurrentPace(component)
            }
        }
    }
}

impl Component {
    pub fn state(&mut self, timer: &Timer) -> ComponentState {
        match *self {
            Component::Title(ref mut component) => ComponentState::Title(component.state(timer)),
            Component::Splits(ref mut component) => ComponentState::Splits(component.state(timer)),
            Component::Timer(ref mut component) => ComponentState::Timer(component.state(timer)),
            Component::PreviousSegment(ref mut component) => {
                ComponentState::PreviousSegment(component.state(timer))
            }
            Component::PossibleTimeSave(ref mut component) => {
                ComponentState::PossibleTimeSave(component.state(timer))
            }
            Component::SumOfBest(ref mut component) => {
                ComponentState::SumOfBest(component.state(timer))
            }
            Component::Graph(ref mut component) => ComponentState::Graph(component.state(timer)),
            Component::Text(ref mut component) => ComponentState::Text(component.state()),
            Component::TotalPlaytime(ref mut component) => {
                ComponentState::TotalPlaytime(component.state(timer))
            }
            Component::CurrentPace(ref mut component) => {
                ComponentState::CurrentPace(component.state(timer))
            }
        }
    }
}
