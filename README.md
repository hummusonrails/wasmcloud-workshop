# wasmCloud Distributed Architecture Workshop

![Couchbase Capella](https://img.shields.io/badge/Couchbase_Capella-Enabled-red)
[![License: MIT](https://cdn.prod.website-files.com/5e0f1144930a8bc8aace526c/65dd9eb5aaca434fac4f1c34_License-MIT-blue.svg)](/LICENSE)

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/hummusonrails/wasmcloud-workshop)

Welcome to the wasmCloud Distributed Architecture Workshop! In this session, you'll learn how to design, build, and deploy a distributed system using wasmCloud’s actor-based model, with a focus on scaling across cloud and edge environments.

This workshop introduces you to wasmCloud, a platform designed for building highly distributed, scalable applications. You will gain insights into how to:

* Design a multi-actor architecture to handle distributed workloads.
* Deploy actors and capabilities across different environments efficiently.
* Integrate external services, such as Couchbase, into a wasmCloud system.
* Scale dynamically across both cloud and edge devices.

During this workshop, you will:

1. Deploy a modular system using wasmCloud actors.
2. Integrate Couchbase using wasmCloud’s capability provider model.
3. Run your own edge node inside a GitHub Codespace, connecting to a shared cloud wasmCloud instance.
4. Scale your applications dynamically across multiple hosts and environments.

## Prerequisites

To complete this workshop, you will need:

* A GitHub account: You will use this to access the workshop materials and deploy your own edge node on GitHub Codespaces.
* A Couchbase Capella account: You will use this to access a shared Couchbase cloud database for the workshop.

## Workshop Outline

This workshop is divided into the following sections:

1. Setup and Environment Initialization
   * Fork this repository and launch a GitHub Codespace.
   * Start a local wasmCloud host inside your Codespace.
   * Connect your Codespace’s wasmCloud instance to the shared cloud wasmCloud control plane.
   * Verify that your edge node is correctly registered in the central wasmCloud instance.

2. Deploying Your First wasmCloud Actor
   * Run a `hello world` actor to understand how wasmCloud applications work.
   * Use the `wash` CLI to interact with your actors.
   * Deploy the actor inside your Codespace and test it locally.

3. Connecting to the Shared wasmCloud Instance
   * Modify your actor to communicate with the central wasmCloud instance.
   * Use NATS messaging to exchange data between edge nodes and cloud services.
   * Verify that your actor can communicate across multiple wasmCloud hosts.

4. Integrating Couchbase with wasmCloud
   * Deploy the Couchbase capability provider in the shared wasmCloud instance.
   * Modify your actor to store and retrieve data from Couchbase Capella.
   * Send and fetch data from the cloud database while running your actor at the edge.

5. Scaling and Distributed Messaging
   * Deploy multiple actors across different hosts (your Codespace and the cloud instance).
   * Use NATS messaging to send requests between actors running on different machines.
   * Scale your actor dynamically between cloud and edge nodes.