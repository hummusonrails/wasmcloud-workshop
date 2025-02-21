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
  * Verify that your wasmCloud host is running correctly.
2. Deploying Your First wasmCloud Actor
  * Run a `hello world` actor to understand how wasmCloud applications work.
  * Use the `wash` CLI to interact with your actors.
3. Integrating Couchbase with wasmCloud
  * Deploy a wasmCloud capability provider for Couchbase.
  * Connect your actor to Couchbase Capella for data operations.
4. Scaling Across Cloud and Edge
  * Set up a multi-host deployment.
  * Real-time communication between cloud and edge nodes using NATS.