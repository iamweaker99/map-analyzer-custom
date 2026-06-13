import { NextResponse } from "next/server";
import type { NextRequest } from "next/server";
import { createHash } from "crypto";

function computeExpectedValue(): string {
    const password = process.env.ANALYZER_PASSWORD;
    const secret = process.env.AUTH_SECRET;
    if (!password || !secret) return "";
    return createHash("sha256")
        .update(password + ":" + secret)
        .digest("hex");
}

export function middleware(request: NextRequest) {
    const { pathname } = request.nextUrl;

    // Allow login page, static assets, and Next.js internals
    if (
        pathname === "/login" ||
        pathname.startsWith("/_next/") ||
        pathname.startsWith("/static/") ||
        pathname === "/favicon.ico"
    ) {
        return NextResponse.next();
    }

    const sessionCookie = request.cookies.get("session")?.value;
    const expected = computeExpectedValue();

    if (!sessionCookie || sessionCookie !== expected) {
        const loginUrl = new URL("/login", request.url);
        loginUrl.searchParams.set("redirect", pathname);
        return NextResponse.redirect(loginUrl);
    }

    return NextResponse.next();
}

export const config = {
    matcher: [
        // Match all routes except static files
        "/((?!_next/static|_next/image|favicon.ico).*)",
    ],
};
