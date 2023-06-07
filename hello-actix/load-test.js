import http from 'k6/http';
import { sleep } from 'k6';

export const options = {
  vus: 200,
  duration: '1m',
};

export default function() {
  http.get('http://hello-actix.eastus.cloudapp.azure.com/weatherforecast');
}

