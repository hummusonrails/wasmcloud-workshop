# Step by Step Guide for Running the Workshop

## Pre-Workshop Setup

1. Choose a cloud provider for the central wasmCloud

You will need to deploy the central wasmCloud instance somewhere online so all attendees can connect their edge nodes to it.

Preferred options:

* [AWS](https://aws.amazon.com)
* [GCP](https://cloud.google.com)
* [Azure](https://azure.microsoft.com)

For a quick and speedy setup, use Fly.io or Render.

2. Deploy the Central wasmCloud Instance

Once you've chosen a cloud provider, deploy the central wasmCloud instance using the following steps.

| Cloud Provider | Steps |
|----------------|-------|
| **GCP VM**     | 1. Log in to the [Google Cloud Console](https://console.cloud.google.com) and select your project. <br> 2. Create a new VM Instance: Navigate to **Compute Engine > VM Instances** and click **"Create Instance"**. Fill in the details: **Name:** wasmcloud-central, **Region/Zone:** choose one close to you (e.g., us-central1), **Machine type:** e2-standard-2 (2 vCPUs, 8GB RAM), **Boot disk:** select **Ubuntu 22.04 LTS**, and enable **Allow HTTP and HTTPS traffic** under Firewall. Then click **"Create"**. <br> 3. Connect to your VM via SSH: Once the VM is running, click the **"SSH"** button in the console to open a terminal window. <br> 4. Install required packages: Run the following commands to update packages and install Docker, Docker Compose, Git, curl, and net-tools: <br> ```bash<br> sudo apt update && sudo apt upgrade -y<br> sudo apt install -y docker.io docker-compose git curl net-tools<br> ``` <br> Add your user to the Docker group: <br> ```bash<br> sudo usermod -aG docker $USER && newgrp docker<br> ``` <br> 5. Clone the Workshop Repository: <br> ```bash<br> git clone https://github.com/YOUR_USERNAME/wasmcloud-workshop.git<br> cd wasmcloud-workshop/central-node<br> ``` <br> 6. Configure environment variables (replace placeholders as needed): <br> ```bash<br> export CENTRAL_WASMCLOUD_URL="nats://YOUR_VM_PUBLIC_IP:4222"<br> export COUCHBASE_HOST="couchbases://your-capella-cluster.cloud.couchbase.com"<br> export COUCHBASE_USERNAME="your_capella_username"<br> export COUCHBASE_PASSWORD="your_capella_password"<br> export COUCHBASE_BUCKET="wasmcloud_workshop"<br> ``` <br> 7. (Optional) Create Firewall Rules (if necessary, and from outside the VM using the GCP CLI on your machine): <br> ```bash<br> gcloud compute firewall-rules create wasmcloud-allow --allow=tcp:4222,tcp:8222,tcp:8080 --source-ranges=0.0.0.0/0 --target-tags=wasmcloud --description="Allow wasmCloud and NATS"<br> ``` <br> 8. Deploy the Central Node: <br> ```bash<br> docker-compose up -d<br> ``` <br> 9. Verify the deployment: <br> ```bash<br> docker ps<br> docker logs wasmcloud_central<br> wash get hosts<br> ``` |

3. Retrieve the NATS Connection URL

Once your wasmCloud instance is deployed, you need to find the NATS URL so attendees can connect.

For GCP, the URL is `nats://your-gcp-vm-instance-ip:4222`.

Save this URL, you will give it to attendees during the workshop.

4. Test the Central wasmCloud Instance

- Verify wasmCloud is Running

After deploying the central wasmCloud instance, verify that wasmCloud is running by using the `wash` CLI.

```bash
wash get hosts
```

- Start the Couchbase capability provider.

```bash
wash ctl start couchbase
```

- Check the Couchbase provider is running.

```bash
wash get providers
```

> [!TIP]
> If anything fails, check logs:

```bash
docker logs wasmcloud_central
```