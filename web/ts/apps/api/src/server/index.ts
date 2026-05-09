import express, { Application } from "express";
import { json } from "body-parser";
import * as api from "./api";
import * as health from "./health";

const app: Application = express();

const STATUS_PATH = "/_status";
const API_PATH = "/api";

app.use(json());

app.use(STATUS_PATH, health.router);
app.use(API_PATH, api.router);

export default app;
