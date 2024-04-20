attribute vec3 position;
attribute vec3 color;
uniform mat4 uMVMatrix;
uniform mat4 uPMatrix;
uniform float uScaleFactor;
varying vec3 vColor;
uniform bool uIsRenderingPoints;
uniform bool uIsRenderingCubes;

void main() {
    if (uIsRenderingCubes) {
        // Apply scaling to the cube vertices based on the octree node's size
        gl_Position = uPMatrix * uMVMatrix * vec4(position, 1.0);
        vColor = color;
    } else if (uIsRenderingPoints) {
        gl_Position = uPMatrix * uMVMatrix * vec4(position, 1.0);
        vColor = color;
        gl_PointSize = 5.0 * uScaleFactor;
    } else {
        // Render XYZ axis lines
        gl_Position = uPMatrix * uMVMatrix * vec4(position, 1.0);
        vColor = color;
    }
}