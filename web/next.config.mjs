/** @type {import('next').NextConfig} */
const nextConfig = {
  outputFileTracingIncludes: {
    "/v1/runs": ["./bin/boxwren"],
  },
};

export default nextConfig;
