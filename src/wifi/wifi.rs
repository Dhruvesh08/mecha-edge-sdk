use env_logger::Env;
use log::{error, info};
use once_cell::sync::OnceCell;
use wifi_ctrl::{
    sta::{self, NetworkResult, ScanResult},
    Result,
};

static LOGGER_INITIALIZED: OnceCell<()> = OnceCell::new();

fn initialize_logger() {
    LOGGER_INITIALIZED.get_or_init(|| {
        env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    });
}

pub struct WifiModule;

impl WifiModule {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_wifi_list(&self) -> Result<Vec<ScanResult>> {
        initialize_logger();
        info!("Starting All Wifi List Function");

        let mut setup = sta::WifiSetup::new()?;

        let proposed_path = format!("/var/run/wpa_supplicant/wlan0");
        setup.set_socket_path(proposed_path);

        let broadcast = setup.get_broadcast_receiver();
        let requester = setup.get_request_client();
        let runtime = setup.complete();

        let (_runtime, wifi_list, _broadcast) = tokio::join!(
            async move {
                if let Err(e) = runtime.run().await {
                    error!("Error: {}", e);
                }
            },
            WifiModule::wifi_list(requester),
            WifiModule::broadcast_listener(broadcast),
        );

        let wifi_list = wifi_list.unwrap();
        Ok(wifi_list)
    }

    async fn wifi_list(requester: sta::RequestClient) -> Result<Vec<ScanResult>> {
        info!("Requesting scan");
        let scan = requester.get_scan().await?;
        info!("Scan complete");
        info!("Shutting down");
        requester.shutdown().await?;
        Ok(scan.to_vec())
    }

    pub async fn get_known_wifi_list() -> Result<Vec<NetworkResult>> {
        initialize_logger();
        info!("Starting Known Wifi List Function");

        let mut setup = sta::WifiSetup::new()?;

        let proposed_path = format!("/var/run/wpa_supplicant/wlan0");
        setup.set_socket_path(proposed_path);

        let broadcast = setup.get_broadcast_receiver();
        let requester = setup.get_request_client();
        let runtime = setup.complete();

        let (_runtime, known_wifi, _broadcast) = tokio::join!(
            async move {
                if let Err(e) = runtime.run().await {
                    error!("Error: {}", e);
                }
            },
            WifiModule::known_wifi(requester),
            WifiModule::broadcast_listener(broadcast),
        );

        let wifi_list = known_wifi.unwrap();
        Ok(wifi_list)
    }

    async fn known_wifi(requester: sta::RequestClient) -> Result<Vec<NetworkResult>> {
        info!("Requesting scan");
        let scan = requester.get_networks().await?;
        info!("Scan complete");
        info!("Shutting down");
        requester.shutdown().await?;
        Ok(scan)
    }

    pub async fn get_connect_wifi(ssid: &str, psk: &str) -> Result<()> {
        initialize_logger();
        info!("Starting Wifi Connection");

        let mut setup = sta::WifiSetup::new()?;

        let proposed_path = format!("/var/run/wpa_supplicant/wlan0");
        setup.set_socket_path(proposed_path);

        let broadcast = setup.get_broadcast_receiver();
        let requester = setup.get_request_client();
        let runtime = setup.complete();

        let (_runtime, connect_wifi, _broadcast) = tokio::join!(
            async move {
                if let Err(e) = runtime.run().await {
                    error!("Error: {}", e);
                }
            },
            WifiModule::connect_wifi(requester, &ssid, &psk),
            WifiModule::broadcast_listener(broadcast),
        );

        let wifi_list = connect_wifi.unwrap();
        Ok(wifi_list)
    }

    async fn connect_wifi(requester: sta::RequestClient, ssid: &str, psk: &str) -> Result {
        let networks = requester.get_networks().await?;

        //if ssid is in known networks, use that network id to connect else create new network id
        for network in networks {
            if network.ssid == ssid {
                info!("Scan for ssid in known networks");
                requester.select_network(network.network_id).await?;
                requester.shutdown().await?;
                return Ok(());
            }
        }

        //if ssid is not in known networks, create new network id
        let network_id = requester.add_network().await?;
        info!("Network id: {}", network_id);
        info!("Setting network ssid");

        //set network ssid
        requester
            .set_network_ssid(network_id, ssid.to_string())
            .await?;

        info!("Setting network psk");

        //set network psk
        requester
            .set_network_psk(network_id, psk.to_string())
            .await?;

        //select newly created network id
        requester.select_network(network_id).await?;

        requester.shutdown().await?;
        Ok(())
    }

    // remove wifi network from known networks using network id
    pub async fn remove_wifi_network(network_id: usize) -> Result<()> {
        info!("Starting Wifi Connection");

        let mut setup = sta::WifiSetup::new()?;

        let proposed_path = format!("/var/run/wpa_supplicant/wlan0");
        setup.set_socket_path(proposed_path);

        let broadcast = setup.get_broadcast_receiver();
        let requester = setup.get_request_client();
        let runtime = setup.complete();

        let (_runtime, remove_wifi, _broadcast) = tokio::join!(
            async move {
                if let Err(e) = runtime.run().await {
                    error!("Error: {}", e);
                }
            },
            WifiModule::remove_wifi(requester, network_id),
            WifiModule::broadcast_listener(broadcast),
        );

        let wifi_list = remove_wifi.unwrap();
        Ok(wifi_list)
    }

    async fn remove_wifi(requester: sta::RequestClient, network_id: usize) -> Result {
        info!("Removing network id: {}", network_id);
        requester.remove_network(network_id).await?;
        requester.shutdown().await?;
        Ok(())
    }

    async fn broadcast_listener(mut broadcast_receiver: sta::BroadcastReceiver) -> Result {
        while let Ok(broadcast) = broadcast_receiver.recv().await {
            info!("Broadcast: {:?}", broadcast);
        }
        Ok(())
    }
}
