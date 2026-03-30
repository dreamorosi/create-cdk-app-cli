#!/usr/bin/env node
import 'source-map-support/register.js';
import { App } from 'aws-cdk-lib';
import { pascalcase-nameStack } from '../lib/lowercase-name-stack.ts';

const app = new App();
new pascalcase-nameStack(app, 'pascalcase-nameStack', {});
