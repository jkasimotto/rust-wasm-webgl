precision mediump float;

varying vec3 vColor;

uniform bool uIsRenderingPoints;
uniform bool uIsRenderingCubes;
uniform bool uIsRenderingDraggablePoint;
uniform bool uIsRenderingSphereSurface;

uniform float uCubeTransparency;
uniform float uDraggablePointTransparency;
uniform float uSphereSurfaceTransparency;

void main() {
    if (uIsRenderingPoints) {
        float distance = length(gl_PointCoord - vec2(0.5, 0.5));
        if (distance > 0.5) discard;
        gl_FragColor = vec4(vColor, 1.0);
    } else if (uIsRenderingCubes) {
        // Set the transparency for the cubes
        gl_FragColor = vec4(1.0, 0.0, 1.0, uCubeTransparency);
    } else if (uIsRenderingDraggablePoint) {
        float distance = length(gl_PointCoord - vec2(0.5, 0.5));
        if (distance > 0.5) discard;
        gl_FragColor = vec4(vColor, uDraggablePointTransparency);
    } else if (uIsRenderingSphereSurface) {
        gl_FragColor = vec4(vColor, uSphereSurfaceTransparency);
    } else {
        // Render XYZ axis lines
        gl_FragColor = vec4(vColor, 1.0);
    }
}