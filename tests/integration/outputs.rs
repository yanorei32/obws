#![cfg(feature = "test-integration")]

use std::time::Duration;

use anyhow::Result;
use obws::events::{Event, OutputState};
use tokio::time;

use crate::{common, wait_for};

#[tokio::test]
async fn outputs() -> Result<()> {
    let client = common::new_client().await?;
    let events = client.events()?;
    let client = client.outputs();

    tokio::pin!(events);

    client.get_virtual_cam_status().await?;

    client.toggle_virtual_cam().await?;
    wait_for!(
        events,
        Event::VirtualcamStateChanged {
            output_state: OutputState::Started,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;
    client.toggle_virtual_cam().await?;
    wait_for!(
        events,
        Event::VirtualcamStateChanged {
            output_state: OutputState::Stopped,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;
    client.start_virtual_cam().await?;
    wait_for!(
        events,
        Event::VirtualcamStateChanged {
            output_state: OutputState::Started,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;
    client.stop_virtual_cam().await?;
    wait_for!(
        events,
        Event::VirtualcamStateChanged {
            output_state: OutputState::Stopped,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;

    client.get_replay_buffer_status().await?;

    client.toggle_replay_buffer().await?;
    wait_for!(
        events,
        Event::ReplayBufferStateChanged {
            output_state: OutputState::Started,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;
    client.toggle_replay_buffer().await?;
    wait_for!(
        events,
        Event::ReplayBufferStateChanged {
            output_state: OutputState::Stopped,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;
    client.start_replay_buffer().await?;
    wait_for!(
        events,
        Event::ReplayBufferStateChanged {
            output_state: OutputState::Started,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;
    client.save_replay_buffer().await?;
    client.get_last_replay_buffer_replay().await?;
    client.stop_replay_buffer().await?;
    wait_for!(
        events,
        Event::ReplayBufferStateChanged {
            output_state: OutputState::Stopped,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;

    Ok(())
}