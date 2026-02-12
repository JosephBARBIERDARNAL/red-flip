import type { Metadata } from "next";
import { Open_Sans, Noto_Serif, Coming_Soon } from "next/font/google";
import "./globals.css";
import { AuthProvider } from "@/context/AuthContext";
import Header from "@/components/layout/Header";
import Footer from "@/components/layout/Footer";

const openSans = Open_Sans({
  variable: "--font-open-sans",
  subsets: ["latin"],
});

const notoSerif = Noto_Serif({
  variable: "--font-noto-serif",
  subsets: ["latin"],
});

const comingSoon = Coming_Soon({
  variable: "--font-coming-soon",
  weight: "400",
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "Red Flip - Rock Paper Scissors",
  description: "Compete in real-time Rock Paper Scissors with Elo ranking",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body
        className={`${openSans.variable} ${notoSerif.variable} ${comingSoon.variable} font-sans antialiased`}
      >
        <AuthProvider>
          <div className="min-h-screen flex flex-col">
            <Header />
            <main className="flex-1">{children}</main>
            <Footer />
          </div>
        </AuthProvider>
      </body>
    </html>
  );
}
