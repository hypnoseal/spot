use yew::prelude::*;
use yew::services::DialogService;
use crate::spots::*;
use crate::svg_builder::*;

pub enum Msg {
    SetNumSpots(usize),
    SetDimension(usize),
    SetMargin(f32),
    Generate,
    New(SpotsGenerator)
}

pub struct SpotsGenerator {
    spots: Spots,
    num_spots: usize,
    dimension: usize,
    margin: f32,
}

impl SpotsGenerator {
    pub fn new() -> SpotsGenerator {
        Self {
            spots: Spots {
                spots: vec![],
                margin: 0.0,
                pointer: 0
            },
            num_spots: 0,
            dimension: 0,
            margin: 0.0,
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct SpotSVGComponentProps {
    spots: Spots,
    dimension: usize,
    margin: f32,
}

struct SpotSVGComponent {
    link: ComponentLink<Self>,
    props: SpotSVGComponentProps,
    svg: Html,
}

impl Component for SpotSVGComponent {
    type Message = Msg;
    type Properties = SpotSVGComponentProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let svg = create_svg(
            props.spots.clone(),
            props.dimension.clone(),
            props.margin.clone()
        ).unwrap().get_html();
        Self {
            link,
            props,
            svg
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            // Msg::New(new_spots) => {
            //     self.props.spots = new_spots.spots;
            //     self.props.dimension = new_spots.dimension;
            //     self.props.margin = new_spots.margin;
            //     false
            // }
            _ => {false}
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let SpotSVGComponentProps {
            spots,
            dimension,
            margin
        } = &self.props;
        match create_svg(
            spots.clone(),
            dimension.clone(),
            margin.clone()
        ) {
            Ok(svg) => {
                svg.get_html()
            }
            Err(e) => {
                DialogService::alert(e.msg);
                let html = html! {
                    <>
                        <h2>{"Error loading SVG!"}</h2>
                    </>
                };
                html
            }
        }
    }
}

#[derive(Properties, Clone)]
struct HomeProps {
    #[prop_or_default]
    spot_svg_component_child: ChildrenWithProps<SpotSVGComponent>,
}

struct Home {
    link: ComponentLink<Self>,
    props: HomeProps,
    spots: Spots,
    num_spots: usize,
    dimension: usize,
}

impl Component for Home {
    type Message = Msg;
    type Properties = HomeProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            spots: Spots::create_random_spots(100, 10.0).unwrap(),
            num_spots: 100,
            dimension: 100
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetNumSpots(value) => {
                self.num_spots = value.clone();
                false
            },
            Msg::SetDimension(value) => {
                self.dimension = value.clone();
                false
            },
            Msg::SetMargin(value) => {
                self.spots.margin = value.clone();
                false
            },
            Msg::Generate => {
                self.spots = {
                    match Spots::create_random_spots(
                        self.num_spots.clone(),
                        self.spots.margin.clone()) {
                            Ok(spots) => spots,
                            Err(e) => {
                                DialogService::alert(e.msg);
                                self.spots.clone()
                            }
                    }
                };
                true
            },
            _ => {false}
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let link = &self.link;
        let update_numspots = link.callback(|e: InputData| {
            Msg::SetNumSpots(e.value.parse::<usize>().unwrap())
        });
        let update_dimension = link.callback(|e: InputData| {
            Msg::SetDimension(e.value.parse::<usize>().unwrap())
        });
        let update_margin = link.callback(|e: InputData| {
            Msg::SetMargin(e.value.parse::<f32>().unwrap())
        });
        let generate_spot = self.link.callback( |_| {
            Msg::Generate
        });

        let spot_svg_component_props = yew::props!(SpotSVGComponentProps {
            spots: self.spots.clone(),
            dimension: self.dimension.clone(),
            margin: self.spots.margin.clone(),
        });

        let html = html! {
            <div>
                <h1>{ "Welcome to Spot Art!" }</h1>
                <div>
                    <p>{format!("Numspots is: {}", self.num_spots.clone())}</p>
                    <p>{format!("Dimension is: {}", self.dimension.clone())}</p>
                    <p>{format!("Margin is: {}", self.spots.margin.clone())}</p>
                </div>
                <form>
                    <div>
                        <label for="num_spots">{"Enter the number of spots:"}</label>
                        <input type="number" name="num_spots" id="num_spots" required=true oninput={update_numspots} />
                    </div>
                    <div>
                        <label for="dimension">{"Enter the dimension:"}</label>
                        <input type="number" name="dimension" id="dimension" required=true oninput={update_dimension} />
                    </div>
                    <div>
                        <label for="margin">{"Enter the margin:"}</label>
                        <input type="number" name="margin" id="margin" required=true oninput={update_margin} />
                    </div>
                    <div>
                        <button type="button" onclick={generate_spot}>{"Generate Random Spot Art"}</button>
                    </div>
                </form>
                <SpotSVGComponent with spot_svg_component_props />
            </div>
        };
        html
    }
}

struct App {
    link: ComponentLink<Self>
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let html = html! {
            <>
                <Home />
            </>
        };
        html
    }
}

pub fn start() {
    yew::start_app::<App>();
}
