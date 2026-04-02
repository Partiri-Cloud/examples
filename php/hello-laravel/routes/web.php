<?php

use Illuminate\Support\Facades\Route;

Route::get('/', function () {
    return response()->json(['message' => 'Hello from Laravel on Partiri!']);
});

Route::get('/health', function () {
    return response()->json(['status' => 'ok']);
});
