import Link from "next/link";

export default function CtaSection() {
  return (
    <section className="py-16 px-4 bg-brand-50">
      <div className="max-w-3xl mx-auto text-center">
        <h2 className="font-serif text-3xl font-bold text-brand-800 mb-4">
          Ready to Compete?
        </h2>
        <p className="text-gray-600 mb-8">
          Create an account, find a match, and start climbing the ranks.
        </p>
        <Link
          href="/register"
          className="inline-block px-8 py-3 bg-brand-600 text-white font-bold rounded-lg hover:bg-brand-500 transition-colors"
        >
          Play Now
        </Link>
      </div>
    </section>
  );
}
