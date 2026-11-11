import React from "react";

interface BoxwrenLogoProps {
  size?: number;
  className?: string;
  withGlow?: boolean;
}

export default function BoxwrenLogo({
  size = 28,
  className = "",
  withGlow = false,
}: BoxwrenLogoProps) {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 512 512"
      width={size}
      height={size}
      fill="none"
      className={className}
      aria-hidden="true"
      style={{
        display: "inline-block",
        verticalAlign: "middle",
        flexShrink: 0,
        filter: withGlow ? "drop-shadow(0 0 12px rgba(74, 222, 128, 0.45))" : undefined,
      }}
    >
      <defs>
        <radialGradient id="bw-bg" cx="50%" cy="30%" r="80%">
          <stop offset="0%" stopColor="#14241b" />
          <stop offset="60%" stopColor="#08100c" />
          <stop offset="100%" stopColor="#040806" />
        </radialGradient>
        <linearGradient id="bw-stroke" x1="0" y1="0" x2="512" y2="512" gradientUnits="userSpaceOnUse">
          <stop offset="0%" stopColor="rgba(74, 222, 128, 0.55)" />
          <stop offset="50%" stopColor="rgba(255, 255, 255, 0.16)" />
          <stop offset="100%" stopColor="rgba(74, 222, 128, 0.10)" />
        </linearGradient>
        <linearGradient id="bw-top" x1="128" y1="120" x2="384" y2="240" gradientUnits="userSpaceOnUse">
          <stop offset="0%" stopColor="#86efac" />
          <stop offset="50%" stopColor="#4ade80" />
          <stop offset="100%" stopColor="#22c55e" />
        </linearGradient>
        <linearGradient id="bw-left" x1="128" y1="200" x2="256" y2="400" gradientUnits="userSpaceOnUse">
          <stop offset="0%" stopColor="#16a34a" />
          <stop offset="100%" stopColor="#064e3b" />
        </linearGradient>
        <linearGradient id="bw-right" x1="384" y1="200" x2="256" y2="400" gradientUnits="userSpaceOnUse">
          <stop offset="0%" stopColor="#22c55e" />
          <stop offset="100%" stopColor="#0f766e" />
        </linearGradient>
        <linearGradient id="bw-core" x1="210" y1="200" x2="300" y2="300" gradientUnits="userSpaceOnUse">
          <stop offset="0%" stopColor="#ffffff" />
          <stop offset="40%" stopColor="#a7f3d0" />
          <stop offset="100%" stopColor="#34d399" />
        </linearGradient>
        <filter id="bw-glow-filter" x="-20%" y="-20%" width="140%" height="140%">
          <feGaussianBlur stdDeviation="12" result="blur" />
          <feComposite in="SourceGraphic" in2="blur" operator="over" />
        </filter>
      </defs>

      {/* Enclave Squircle Base */}
      <rect x="24" y="24" width="464" height="464" rx="112" fill="url(#bw-bg)" stroke="url(#bw-stroke)" strokeWidth="8" />

      {/* Ambient Internal Phosphor Radiation */}
      <circle cx="256" cy="256" r="130" fill="#4ade80" opacity="0.14" filter="url(#bw-glow-filter)" />

      {/* Outer Isometric Enclave Shell */}
      <g transform="translate(0, -8)">
        <polygon points="256,112 376,180 256,248 136,180" fill="url(#bw-top)" />
        <polygon points="136,180 256,248 256,380 136,312" fill="url(#bw-left)" />
        <polygon points="256,248 376,180 376,312 256,380" fill="url(#bw-right)" />

        {/* Recessed Guest Chamber Cutout */}
        <polygon points="256,156 342,204 256,252 170,204" fill="#040906" opacity="0.94" />
        <polygon points="170,204 256,252 256,344 170,296" fill="#020504" opacity="0.96" />
        <polygon points="256,252 342,204 342,296 256,344" fill="#030705" opacity="0.95" />

        {/* Kinetic Wren Flight Prism Core */}
        <g filter="url(#bw-glow-filter)">
          <polygon points="256,204 212,252 256,268" fill="#10b981" />
          <polygon points="256,204 300,252 256,268" fill="#34d399" />
          <polygon points="256,184 274,212 238,212" fill="url(#bw-core)" />
          <polygon points="256,268 244,304 268,304" fill="#059669" />
        </g>

        {/* Isolation Coordinate Beams */}
        <circle cx="256" cy="112" r="6" fill="#ffffff" />
        <circle cx="376" cy="180" r="5" fill="#86efac" />
        <circle cx="136" cy="180" r="5" fill="#86efac" />
        <circle cx="256" cy="380" r="6" fill="#4ade80" />
        <circle cx="256" cy="184" r="4" fill="#ffffff" filter="url(#bw-glow-filter)" />
      </g>
    </svg>
  );
}
