use yew::prelude::*;


#[derive(Clone, PartialEq)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
//    on_click: Callback<Video>
}

#[function_component(VideosList)]
fn videos_list(VideosListProps { videos }: &VideosListProps) -> Html {
    videos.iter().map(|video| html! {
        <p key={video.id}>{format!("{}: {}", video.speaker, video.title)}</p>
    }).collect()
}

#[function_component(App)]
fn app() -> Html {
    let videos = vec![
        Video {
            id: 1,
            title: "Test video".to_string(),
            speaker: "Bogusław Łęcina".to_string(),
            url: "https://youtube.com/PsaFvLr8t4E".to_string()
        },
        Video {
            id: 2,
            title: "Test video".to_string(),
            speaker: "Bogusław Łęcina".to_string(),
            url: "https://youtube.com/PsaFvLr8t4E".to_string()
        },
        Video {
            id: 3,
            title: "Test video".to_string(),
            speaker: "Bogusław Łęcina".to_string(),
            url: "https://youtube.com/PsaFvLr8t4E".to_string()
        }
    ];

    html! {
        <>
            <h1>{ "Hello World" }</h1>
            <VideosList videos={videos}/>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
