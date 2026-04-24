import 'reflect-metadata';
import { NestFactory } from '@nestjs/core';
import { AppModule } from './app.module';

async function bootstrap(): Promise<void> {
  const app = await NestFactory.create(AppModule);
  const port = parseInt(process.env.PORT ?? '10000', 10);
  await app.listen(port);
  console.log(`Server listening on port ${port}`);
}

bootstrap();
