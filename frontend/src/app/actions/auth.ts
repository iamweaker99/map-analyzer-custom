"use server";

import { cookies } from "next/headers";
import { redirect } from "next/navigation";
import { createHash } from "crypto";

function computeCookieValue(password: string, secret: string): string {
    return createHash("sha256")
        .update(password + ":" + secret)
        .digest("hex");
}

export async function login(formData: FormData) {
    const password = formData.get("password") as string;
    const redirectTo =
        (formData.get("redirect") as string) || "/";

    const expectedPassword = process.env.ANALYZER_PASSWORD;
    const secret = process.env.AUTH_SECRET;

    if (!expectedPassword || !secret) {
        redirect("/login?error=Server+not+configured+for+authentication");
    }

    if (password !== expectedPassword) {
        redirect(
            "/login?error=Invalid+password&redirect=" +
                encodeURIComponent(redirectTo),
        );
    }

    const cookieValue = computeCookieValue(expectedPassword, secret);

    const cookieStore = await cookies();
    cookieStore.set("session", cookieValue, {
        httpOnly: true,
        secure: true,
        sameSite: "lax",
        path: "/",
    });

    redirect(redirectTo);
}
