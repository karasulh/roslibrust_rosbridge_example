use log::*;
use roslibrust::ClientHandle;
use std::time::Duration;

roslibrust_codegen_macro::find_and_generate_ros_messages!("/Users/mbkara/Documents/_3_Projects/_15_ROS2/ros2_common_interfaces/std_msgs",
                                                          "/Users/mbkara/Documents/_3_Projects/_15_ROS2/humble_docker/ros2_ws/src/trial_interfaces");

/// To run this example a rosbridge websocket server should be running at the deafult port (9090).
#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), anyhow::Error> {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .without_timestamps() // required for running wsl2
        .init()
        .unwrap();

    let client = ClientHandle::new("ws://localhost:9090").await?;
    let publisher = client.advertise::<std_msgs::Int32>("topic2").await?;
    let publisher_number = client.advertise::<trial_interfaces::Numbers>("topic4").await?;

    let subscriber = client.subscribe::<std_msgs::Int32>("topic").await?;
    info!("Successfully subscribed to topic: topic");

    let subscriber_number  = client.subscribe::<trial_interfaces::Numbers>("topic3").await?;
    info!("Successfully subscribed to topic: topic3");

    let mut i:i32 = 0;

    tokio::spawn(async move{
        loop{
            let msg = subscriber.next().await;
            info!("Got msg: {:?}", msg);

            let msg_number = subscriber_number.next().await;
            info!("Got msg Number: {:?}", msg_number);
        
            //handle_message(msg).await;
            tokio::spawn(handle_message_number(msg_number));
        }
    });

    loop {

        // let msg = rx.next().await;
        // info!("Got msg: {:?}", msg);

        //let msg_number = rx_number.next().await;
        //info!("Got msg Number: {:?}", msg_number);

        i+=1;
        let msg = std_msgs::Int32{
            data: i
        };
        //info!("About to publish");
        let result = publisher.publish(msg).await;
        match result {
            Ok(()) => {
                info!("Published msg!");
            }
            Err(e) => {
                error!("Failed to publish msg: {e}");
            }
        }

        let msg_number = trial_interfaces::Numbers{
            number1: i,
            number2: i as f32*1.2,
            number3: std_msgs::Int16{
                data: i as i16*2
            }
        };
        //info!("About to publish number");
        let result = publisher_number.publish(msg_number).await;
        match result {
            Ok(()) => {
                info!("Published msg number!");
            }
            Err(e) => {
                error!("Failed to publish msg number: {e}");
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }

}

// subscription message handler
async fn handle_message_number(msg: trial_interfaces::Numbers) {
    // Simulate processing time or some heavy computation
    tokio::time::sleep(Duration::from_millis(50)).await;
    info!("Processed message: {:?}", msg);
}