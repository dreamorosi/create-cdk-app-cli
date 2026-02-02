#!/usr/bin/env node
import 'source-map-support/register';
import { App } from 'aws-cdk-lib';
import { BiomeTestStack } from '../lib/biometest-stack.js';

const app = new App();
new BiomeTestStack(app, 'BiomeTestStack', {});
