import json

from django.http import HttpRequest, HttpResponse, JsonResponse


def index(request: HttpRequest) -> JsonResponse:
    return JsonResponse({'message': 'Hello from Django on Partiri!'})


def health(request: HttpRequest) -> JsonResponse:
    return JsonResponse({'status': 'ok'})
