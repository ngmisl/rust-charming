use charming::theme::Theme::Chalk;
use charming::{
    component::{Legend, Title},
    element::ItemStyle,
    series::{Pie, PieRoseType},
    Chart, ImageRenderer,
};
use dotenv::dotenv;
use pyroscope::PyroscopeAgent;
use pyroscope_pprofrs::{pprof_backend, PprofConfig};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("API_KEY").unwrap();
    let agent = PyroscopeAgent::builder("https://ingest.pyroscope.cloud", "myapp-profile")
        .backend(pprof_backend(PprofConfig::new().sample_rate(100)))
        .tags(vec![("env", "dev")])
        .auth_token(api_key)
        .build()?;
    let agent_running = agent.start().unwrap();
    let chart = Chart::new()
        .title(Title::new().text("Mintmons Level 1"))
        .legend(Legend::new().top("bottom"))
        .series(
            Pie::new()
                .name("Mintmons Level 1")
                .rose_type(PieRoseType::Radius)
                .radius(vec!["50", "250"])
                .center(vec!["50%", "50%"])
                .item_style(ItemStyle::new().border_radius(16))
                .data(vec![
                    (40.0, "Firery"),
                    (40.0, "Windry"),
                    (40.0, "Stoney"),
                    (40.0, "Watery"),
                    (53.833_333_33, "Beary"),
                    (46.666_666_67, "Cochlefy"),
                    (54.166_666_67, "Flederfy"),
                    (53.333_333_33, "Focafy"),
                    (55.833_333_33, "Formify"),
                    (44.166_666_67, "Kupufy"),
                    (46.666_666_67, "Oursinfy"),
                    (51.0, "Phoenexy"),
                    (80.0, "Daemon"),
                    (64.166_666_67, "Scorpy"),
                    (71.666_666_67, "Axoloty"),
                    (72.5, "Adter"),
                ]),
        );

    let mut renderer = ImageRenderer::new(1000, 800).theme(Chalk);
    let _ = renderer.save(&chart, "./mintmons.svg");
    let agent_ready = agent_running.stop()?;
    agent_ready.shutdown();

    Ok(())
}
