# Step by Step Guide for Running the Workshop

## Pre-Workshop Setup

1. Choose a cloud provider for the central wasmCloud

You will need to deploy the central wasmCloud instance somewhere online so all attendees can connect their edge nodes to it.

Preferred options:

* [Fly.io](https://fly.io)
* [Render](https://render.com)
* [AWS](https://aws.amazon.com)
* [GCP](https://cloud.google.com)
* [Azure](https://azure.microsoft.com)

For a quick and speedy setup, use Fly.io or Render.

2. Deploy the Central wasmCloud Instance

Once you've chosen a cloud provider, deploy the central wasmCloud instance using the following steps.

| Cloud Provider | Steps |
|----------------|-------|
| **Fly.io**     | 1. Install the Fly CLI: <br> ```bash <br> curl -L https://getfly.io/flyctl.sh \| sh <br> ``` <br> 2. Authenticate: <br> ```bash <br> fly auth login <br> ``` <br> 3. Create a new Fly app: <br> ```bash <br> fly launch --name wasmcloud-central <br> ``` <br> 4. Deploy the app: <br> ```bash <br> fly deploy <br> ``` |
| **Render**     | 1. Sign up at [Render](https://render.com). <br> 2. Create a new web service and connect it to your GitHub repo. <br> 3. Set the build command to: <br> ```bash <br> docker compose up <br> ``` <br> 4. Deploy. |

3. Retrieve the NATS Connection URL

Once your wasmCloud instance is deployed, you need to find the NATS URL so attendees can connect.

For Fly.io, the URL is `nats://your-app.fly.dev:4222`.

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