import { Controller, Get, HttpCode } from '@nestjs/common';

@Controller()
export class AppController {
  @Get()
  root(): string {
    return 'Hello from NestJS on Partiri!';
  }

  @Get('health')
  @HttpCode(200)
  health(): { status: string } {
    return { status: 'ok' };
  }
}
