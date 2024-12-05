import { ZKGeoPoint } from 'zklocus';
import { ZKThreePointPolygon } from 'zklocus';


const latitude = 40.7128;
const longitude = -74.0060;
const zkGeoPoint = new ZKGeoPoint(latitude, longitude);



const polygon = new ZKThreePointPolygon(
  { latitude: 40.7128, longitude: -74.0060 },
  { latitude: 40.7129, longitude: -74.0061 },
  { latitude: 40.7130, longitude: -74.0062 }
);

const proof = await zkGeoPoint.Prove.inPolygon(polygon);