import { Router } from "express";

import * as v0 from "./v0";

const _router: Router = Router();

_router.use("/v0", v0.router);

export const router: Router = _router;
