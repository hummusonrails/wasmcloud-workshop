# Workshop Agenda Breakdown

## Agenda Overview

| Time          | Activity                                   | Key Learning                                      |
|--------------|-------------------------------------------|--------------------------------------------------|
| 0:00 - 0:10  | Welcome + Intro to wasmCloud             | Understanding actor-based distributed systems    |
| 0:10 - 0:25  | Setting Up Your Edge Node                | Attendees fork the repo and open a Codespace    |
| 0:25 - 0:40  | Deploying the First Actor                | Running the Hello World actor                   |
| 0:40 - 1:00  | Connecting to the Shared Cloud wasmCloud | Linking local actors to the control plane       |
| 1:00 - 1:20  | Integrating Couchbase into the Actor     | Using the Couchbase provider to store/retrieve data |
| 1:20 - 1:40  | Scaling and Distributed Messaging        | Deploying multiple actors across cloud & edge   |
| 1:40 - 2:00  | Q&A + Wrap-Up                            | Discussion, best practices, next steps          |

---

## 0:00 - 0:10 | Welcome + Introduction to wasmCloud  

### **Facilitator Instructions**
- Welcome attendees and briefly introduce the workshop structure.
- Provide an overview of **wasmCloud** and why it's useful for **distributed systems**.
- Explain the **actor-based model** and how it differs from traditional services.
- Highlight the **goal of the workshop**: to build and scale a **distributed architecture using wasmCloud and Couchbase**.

### **Key Talking Points**
- What is **WebAssembly** and why does it matter for distributed systems?
- How **wasmCloud** enables **portable, scalable, and secure applications**.
- The difference between **actors and providers** in wasmCloud.
- Real world use cases where **wasmCloud is a great fit**.

---

## 0:10 - 0:25 | Setting Up Your Edge Node  

### **Facilitator Instructions**
- Guide attendees through **forking the repository** and launching **GitHub Codespaces**.
- Walk them through the **pre-configured devcontainer setup**.
- Ensure that each participant has their own **wasmCloud host running inside Codespaces**.
- Validate the installation by running:
  ```bash
  wash up -d
  wash get hosts
  ```
- Ask attendees to confiirm that their **Codespace is correctly registered**.

## 0:25 - 0:40 | Deploying the First Actor

### **Facilitator Instructions**
- Introduce **the concept of actors** in wasmCloud.
- Guide attendees through deploying the **Hello World actor**.
  ```bash
  wash app deploy hello-world
  ```
- Demonstrate how to **call the actor** and receive a response.
  ```bash
  wash call actor-id HelloWorld '{"name": "Hola, Barcelona!"}'
  ```
- Explain **how actors remain stateless and interact via providers**.

### Discussion Question
- How does the **actor model** differ from traditional microservices?

## 0:40 - 1:00 | Connecting to the Shared Cloud wasmCloud

### **Facilitator Instructions**
- Explain how **NATS messaging** makes **multi-host communication** possible.
- Have attendees **connect their Codespace wasmCloud instance** to the **shared cloud wasmCloud control plane**:
  ```bash
  wash up -d --nats-connect <cloud-nats-url>
  ```
- Verify that their **Codespace is registered as an edge node**:
  ```bash
    wash get hosts
    ```
- Demonstrate how actors can be **linked across hosts**.

### Key Learning Points
- How **NATS connects distributed wasmCloud instances**.
- The **benefits of edge to cloud actor deployments**.

## 1:00 - 1:20 | Integrating Couchbase into the Actor

### **Facilitator Instructions**
- Introduce **capability providers** in wasmCloud.
- Deploy the **Couchbase capability provider**.
- Modify the actor to **store and retrieve data** from Couchbase Capella.
- Test reading and writing data to Couchbase.

### Key Learning Points
- How **capability providers** enable external services.
- Why **Couchbase is an ideal data solution for distributed wasmCloud deployments**.

## 1:20 - 1:40 | Scaling and Distributed Messaging

### **Facilitator Instructions**
- Guide attendees through **deploying multiple actors** across:
  - Their Codespace instance
  - The shared cloud wasmCloud instance
- Demonstrate how **actors in different locations communicate**:
  ```bash
  wash call <actor-1> SendMessage '{ "to": "<actor-2>", "message": "Hello from my edge node!" }'
  ```
- Ask attendees to **test cross-node messaging**.

### Key Learning Points
- How to **scale actors dynamically** between edge and cloud.
- The role of **NATS messaging** in a **distributed wasmCloud system**.

## 1:40 - 2:00 | Q&A + Wrap-Up

### **Facilitator Instructions**
- Recap the **key takeaways from the workshop**.
- Open the floor for **questions and discussion**.