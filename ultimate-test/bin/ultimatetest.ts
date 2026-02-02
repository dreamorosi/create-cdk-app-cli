#!/usr/bin/env node
import "source-map-support/register";
import { App } from "aws-cdk-lib";
import { UltimateTestStack } from "../lib/ultimatetest-stack.js";

const app = new App();
new UltimateTestStack(app, "UltimateTestStack", {});