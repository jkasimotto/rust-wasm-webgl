attribute vec3 position;
attribute vec3 color;

uniform mat4 uMVMatrix;
uniform mat4 uPMatrix;
uniform float uScaleFactor;
uniform bool uIsRenderingPoints;
uniform bool uIsRenderingCubes;
uniform bool uIsRenderingDraggablePoint;
uniform bool uIsRenderingSphereSurface;

varying vec3 vColor;

void main() {
    gl_Position = uPMatrix * uMVMatrix * vec4(position, 1.0);
    vColor = color;

    if (uIsRenderingCubes) {
        // No additional processing needed for cubes
    } else if (uIsRenderingPoints) {
        gl_PointSize = 5.0 * uScaleFactor;
    } else if (uIsRenderingDraggablePoint) {
        gl_PointSize = 10.0 * uScaleFactor; // Larger size for the draggable point
    } else if (uIsRenderingSphereSurface) {
        // No additional processing needed for the sphere surface
    } else {
        // Render XYZ axis lines
    }
}